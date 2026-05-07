//! Startup auto-check + active-window toast notification for GitHub releases.
//!
//! Listens for `GithubUpdateState` transitions; when a check resolves to
//! `UpdateAvailable`, surfaces one persistent toast in the active window.
//! Idempotent within a session — at most one toast per warp launch. Dismissing
//! the toast leaves no persistent state, so the next launch re-checks and
//! re-notifies if the update is still pending. This satisfies the desired UX:
//! "click [Update] → goes to Settings → About; click [Later] / dismiss → next
//! launch reminds again".
//!
//! Persists `last_check.json` so the Settings "last checked" line and the
//! menu-badge red dot survive restarts without forcing a fresh network round
//! trip every boot.

use std::time::Duration;

use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use warp_core::paths;
use warpui::r#async::Timer;
use warpui::windowing::WindowManager;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

const MAX_LAST_CHECK_BYTES: u64 = 16 * 1024;

use crate::view_components::DismissibleToast;
use crate::workspace::{ToastStack, WorkspaceAction};

use super::GithubUpdateState;

/// Delay between init and first GitHub check. Long enough for the main window
/// to be fully constructed (so `WindowManager::active_window()` is populated)
/// without making the user wait noticeably.
const STARTUP_DELAY: Duration = Duration::from_secs(5);
const LAST_CHECK_FILE: &str = "github_last_check.json";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedLastCheck {
    at_secs: u64,
    status: PersistedStatus,
}

/// Persisted check outcome. Intentionally coarse — `UpdateAvailable` is *not*
/// stored, because the live `GithubUpdateState` (seeded at boot from the
/// release cache, then refreshed by the 5s startup check) is the single source
/// of truth for "is an update pending?". Persisting that bit independently
/// caused the menu badge to remain red on the first launch after install,
/// until the next check overwrote `last_check.json`.
#[derive(Clone, Debug, Serialize, Deserialize)]
enum PersistedStatus {
    UpToDate,
    Error,
}

pub(crate) struct UpdateNotificationModel {
    shown_in_session: bool,
    last_check_at: Option<u64>,
}

impl UpdateNotificationModel {
    fn new(ctx: &mut ModelContext<Self>) -> Self {
        let state_handle = GithubUpdateState::handle(ctx);
        ctx.subscribe_to_model(&state_handle, Self::on_state_change);

        // Restore the persisted last-check. The companion seeding of
        // `GithubUpdateState` from the release cache happens inside
        // [`GithubUpdateState::new`] so the menu-badge / "last checked"
        // line render correctly from the very first frame.
        let last_check_at = read_last_check().map(|p| p.at_secs);

        // Schedule one-shot check after the main window has had a chance to
        // come up. `force = false` so the 12h cache short-circuits the
        // network on warm restarts; the conditional GET is otherwise free
        // (304 doesn't consume rate budget).
        ctx.spawn(
            async {
                Timer::after(STARTUP_DELAY).await;
            },
            |_self, _, ctx| {
                GithubUpdateState::trigger_check(ctx, false);
            },
        );

        Self {
            shown_in_session: false,
            last_check_at,
        }
    }

    /// Public accessor for the Settings "last checked HH:MM:SS" line. Unix
    /// seconds; render conversion lives in the view.
    pub fn last_check_at(&self) -> Option<u64> {
        self.last_check_at
    }

    fn on_state_change(&mut self, _event: &(), ctx: &mut ModelContext<Self>) {
        let state = GithubUpdateState::as_ref(ctx).clone();

        // Only persist *terminal* check results. Skipping Checking /
        // Downloading / Installing / Idle keeps last_check_at meaningful
        // (= "moment we last knew the answer") rather than ticking on
        // every transient transition. `UpdateAvailable` collapses to
        // `UpToDate` on disk — the live state is what drives the badge.
        let persisted = match &state {
            GithubUpdateState::UpToDate | GithubUpdateState::UpdateAvailable { .. } => {
                Some(PersistedLastCheck {
                    at_secs: now_secs(),
                    status: PersistedStatus::UpToDate,
                })
            }
            GithubUpdateState::Error => Some(PersistedLastCheck {
                at_secs: now_secs(),
                status: PersistedStatus::Error,
            }),
            _ => None,
        };

        if let Some(p) = persisted.as_ref() {
            self.last_check_at = Some(p.at_secs);
            if let Err(err) = write_last_check(p) {
                log::debug!("write last_check cache failed: {err:#}");
            }
            ctx.notify();
        }

        if self.shown_in_session {
            return;
        }
        if let GithubUpdateState::UpdateAvailable { tag, .. } = state {
            self.shown_in_session = true;
            self.push_toast(tag, ctx);
        }
    }

    fn push_toast(&self, tag: String, ctx: &mut ModelContext<Self>) {
        let Some(window_id) = WindowManager::as_ref(ctx).active_window() else {
            log::warn!("auto-update toast skipped: no active window at notify time");
            return;
        };
        let text = warp_i18n::t!("settings-account-update-toast-available").replace("<tag>", &tag);
        let toast = DismissibleToast::<WorkspaceAction>::default(text);
        ToastStack::handle(ctx).update(ctx, |stack, ctx| {
            stack.add_persistent_toast(toast, window_id, ctx);
        });
    }
}

impl Entity for UpdateNotificationModel {
    type Event = ();
}

impl SingletonEntity for UpdateNotificationModel {}

/// Register the notification model. Construction schedules the startup check
/// and subscribes to state transitions.
pub(crate) fn register(ctx: &mut AppContext) {
    ctx.add_singleton_model(UpdateNotificationModel::new);
}

fn last_check_path() -> std::path::PathBuf {
    paths::cache_dir().join(LAST_CHECK_FILE)
}

fn read_last_check() -> Option<PersistedLastCheck> {
    super::read_capped_json(&last_check_path(), MAX_LAST_CHECK_BYTES)
}

fn write_last_check(check: &PersistedLastCheck) -> Result<()> {
    let path = last_check_path();
    let bytes = serde_json::to_vec(check).context("serialize last check")?;
    super::write_json_atomically(&path, &bytes).context("write last check")?;
    Ok(())
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
