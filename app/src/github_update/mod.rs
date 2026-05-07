//! Update checker + installer against the fork's GitHub Releases.
//!
//! Independent from `crate::autoupdate` (which talks to warp.dev's release
//! infrastructure and is unreachable on a fork). Drives the Settings page
//! Version row:
//!   * Shows current version.
//!   * Polls the latest release on `Heartcoolman/warp-cn`.
//!   * Lets the user trigger an in-place download/verify/extract/replace
//!     pipeline (see [`install`]) when the release publishes a signed
//!     tarball + `.minisig`. Falls back to "Open on GitHub" otherwise.
//!
//! Auto-update path is macOS-only; on other targets the install module is
//! not compiled.

use anyhow::{anyhow, bail, Context as _, Result};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::io::{Read as _, Write as _};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering as AtomicOrdering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use warp_core::channel::ChannelState;
use warp_core::paths;
use warpui::r#async::Timer;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

mod auto_check;
#[cfg(target_os = "macos")]
mod install;

pub(crate) use auto_check::{register as register_auto_check, UpdateNotificationModel};

/// Unix-seconds timestamp of the most recent terminal check
/// (UpToDate / UpdateAvailable / Error). `None` until the first check
/// completes on a fresh install. Survives restarts via `auto_check`'s
/// `last_check.json`.
pub(crate) fn last_check_at_secs(app: &AppContext) -> Option<u64> {
    UpdateNotificationModel::as_ref(app).last_check_at()
}

/// True iff the live `GithubUpdateState` says an update is available.
/// Single-sourced from the in-memory state (which itself is seeded at boot
/// from `latest_release.json` via [`restore_state_from_cache`]) so the
/// menu badge can never stick on a stale "available" verdict for a
/// version the user has already installed.
pub(crate) fn pending_update_visible(app: &AppContext) -> bool {
    matches!(
        GithubUpdateState::as_ref(app),
        GithubUpdateState::UpdateAvailable { .. }
    )
}

const REPO_API_URL: &str = "https://api.github.com/repos/Heartcoolman/warp-cn/releases/latest";
pub(crate) const REPO_RELEASES_URL: &str = "https://github.com/Heartcoolman/warp-cn/releases";
const REPO_TAGS_API: &str =
    "https://api.github.com/repos/Heartcoolman/warp-cn/git/matching-refs/tags/";
const REPO_GIT_TAGS_API: &str = "https://api.github.com/repos/Heartcoolman/warp-cn/git/tags/";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const USER_AGENT: &str = "warp-cn-update-check/0.1";
const ACCEPT_HEADER: &str = "application/vnd.github+json";
/// Cache lifetime for SHA→tag resolution. SHA→tag is stable unless the user
/// retags the same commit; 24h re-validation is the right balance for honoring
/// retags without burning the GitHub unauthenticated rate budget (60/h/IP).
const VERSION_CACHE_TTL: Duration = Duration::from_secs(24 * 60 * 60);
const VERSION_CACHE_FILE: &str = "version_resolution.json";

/// Auto-check throttle window. Below this age, an unforced
/// `trigger_check` reuses the cached release without hitting the network at
/// all; conditional `If-None-Match` is still cheap (304 doesn't consume rate
/// budget) but skipping it entirely is friendlier to laptops on flaky
/// hotel wifi.
const AUTO_CHECK_THROTTLE: Duration = Duration::from_secs(12 * 60 * 60);
const LATEST_RELEASE_CACHE_FILE: &str = "latest_release.json";
/// Hard cap on accepted cache file sizes. The real GitHub `/releases/latest`
/// payload for warp-cn sits around 8 KB; 1 MiB is generous headroom while
/// still keeping a runaway-corrupt cache from blocking startup.
const MAX_LATEST_RELEASE_CACHE_BYTES: u64 = 1024 * 1024;

/// Set when a user-initiated `trigger_check(_, true)` arrives while an earlier
/// check is still in flight. The in-flight check's spawn callback honors and
/// clears this flag, re-firing exactly one forced re-check. Module-scoped
/// rather than carried inside the enum because the deferral is purely a
/// scheduling artifact — it does not affect rendering, persistence, or the
/// state machine semantics observable to subscribers.
static PENDING_FORCE_RECHECK: AtomicBool = AtomicBool::new(false);

/// Carrier for a release that has both an installable tarball and its
/// minisign signature. Surfacing this as `Some(_)` from a check tells the
/// UI it can offer "Download & Install"; `None` means fall back to the
/// browser link.
#[derive(Clone, Debug)]
pub struct InstallableRelease {
    pub tag: String,
    pub asset_url: String,
    pub sig_url: String,
}

#[derive(Clone, Debug)]
pub enum GithubUpdateState {
    Idle,
    Checking,
    UpToDate,
    UpdateAvailable {
        tag: String,
        html_url: String,
        installable: Option<InstallableRelease>,
    },
    /// Tarball is being fetched + verified. `downloaded_bytes` is updated
    /// every ~250ms by a ticker that polls a shared atomic written by the
    /// install future's stream loop. `total_bytes` is `None` until the HTTP
    /// response's `Content-Length` is known (almost always present for
    /// GitHub release-asset redirects).
    Downloading {
        tag: String,
        downloaded_bytes: u64,
        total_bytes: Option<u64>,
    },
    /// Verification done; about to swap and relaunch. The current process
    /// will `exit(0)` very shortly after entering this state, so the user
    /// rarely sees this frame.
    Installing {
        tag: String,
    },
    Error,
}

impl GithubUpdateState {
    pub fn new() -> Self {
        // Boot-time restore from the on-disk release cache — keeps the menu
        // badge and Settings/About version row consistent from the very
        // first frame instead of flashing `Idle` for ~5s while the startup
        // timer waits to fire. Doing it here (rather than from
        // [`UpdateNotificationModel::new`]) avoids a re-entrant write to
        // this entity *during* another entity's construction.
        match restore_state_from_cache() {
            Some(CheckResult::UpToDate) => Self::UpToDate,
            Some(CheckResult::UpdateAvailable {
                tag,
                html_url,
                installable,
            }) => Self::UpdateAvailable {
                tag,
                html_url,
                installable,
            },
            None => Self::Idle,
        }
    }

    pub fn register(ctx: &mut AppContext) {
        ctx.add_singleton_model(|_ctx| Self::new());
    }

    /// True iff the binary was built with a baked update public key. UI uses
    /// this to decide whether to surface the install button at all; the
    /// install module enforces the same check defensively before applying.
    pub fn install_supported() -> bool {
        cfg!(target_os = "macos")
            && option_env!("WARP_UPDATE_PUBKEY").is_some_and(|s| !s.is_empty())
    }

    /// Kick off a release check.
    ///
    /// `force = false` is the auto-check path: a non-stale 12h cache short-
    /// circuits the network entirely; a stale-but-present cache still goes
    /// out, but with `If-None-Match` so a 304 doesn't consume rate budget.
    /// `force = true` is the user-clicked-the-button path: always hits the
    /// network (still uses `If-None-Match`, since 304 is free) and on
    /// failure surfaces the error UI rather than silently reusing cache.
    pub fn trigger_check(ctx: &mut AppContext, force: bool) {
        Self::handle(ctx).update(ctx, |state, ctx| {
            if matches!(
                state,
                Self::Checking | Self::Downloading { .. } | Self::Installing { .. }
            ) {
                // A user-initiated `force=true` arriving while an auto-check is
                // already in flight must not be silently dropped — defer it,
                // and the spawn callback below will re-fire once the current
                // check settles.
                if force {
                    PENDING_FORCE_RECHECK.store(true, AtomicOrdering::SeqCst);
                }
                return;
            }

            // We're about to do the requested work; clear any prior deferral.
            if force {
                PENDING_FORCE_RECHECK.store(false, AtomicOrdering::SeqCst);
            }
            *state = Self::Checking;
            ctx.notify();

            ctx.spawn(
                async move { check_for_update(force).await },
                |state, result, ctx| {
                    *state = match result {
                        Ok(CheckResult::UpToDate) => Self::UpToDate,
                        Ok(CheckResult::UpdateAvailable {
                            tag,
                            html_url,
                            installable,
                        }) => Self::UpdateAvailable {
                            tag,
                            html_url,
                            installable,
                        },
                        Err(err) => {
                            log::warn!("GitHub update check failed: {err:#}");
                            Self::Error
                        }
                    };
                    ctx.notify();

                    // Honor a deferred force-recheck (user clicked while the
                    // earlier check was still in flight). Only re-fires when
                    // the flag was specifically set, so persistent network
                    // failure cannot self-loop.
                    if PENDING_FORCE_RECHECK.swap(false, AtomicOrdering::SeqCst) {
                        Self::trigger_check(ctx, true);
                    }
                },
            );
        });
    }

    /// Kick off the download/verify/extract/replace pipeline. macOS-only;
    /// no-ops on other platforms (and the install button is hidden).
    #[cfg(target_os = "macos")]
    pub fn trigger_install(ctx: &mut AppContext, target: InstallableRelease) {
        Self::handle(ctx).update(ctx, |state, ctx| {
            if matches!(state, Self::Downloading { .. } | Self::Installing { .. }) {
                return;
            }
            let tag = target.tag.clone();
            let progress = Arc::new(AtomicU64::new(0));
            let total = Arc::new(AtomicU64::new(0));
            let done = Arc::new(AtomicBool::new(false));

            *state = Self::Downloading {
                tag: tag.clone(),
                downloaded_bytes: 0,
                total_bytes: None,
            };
            ctx.notify();

            let install_target = target.clone();
            let install_tag = tag.clone();
            let progress_w = progress.clone();
            let total_w = total.clone();
            let done_w = done.clone();
            ctx.spawn(
                async move {
                    let r = install::run_install(install_target, progress_w, total_w).await;
                    done_w.store(true, AtomicOrdering::SeqCst);
                    r
                },
                move |state, result, ctx| {
                    match result {
                        Ok(()) => {
                            *state = Self::Installing { tag: install_tag };
                        }
                        Err(err) => {
                            log::error!("GitHub update install failed: {err:#}");
                            *state = Self::Error;
                        }
                    }
                    ctx.notify();
                },
            );

            // Re-entrant ticker that reflects atomic counters into model
            // state every ~250ms. Stops when `done` flips true (set by the
            // install future just before its callback fires) or when state
            // moves out of Downloading{tag}.
            Self::schedule_progress_tick(ctx, tag, progress, total, done);
        });
    }

    #[cfg(target_os = "macos")]
    fn schedule_progress_tick(
        ctx: &mut ModelContext<Self>,
        tag: String,
        progress: Arc<AtomicU64>,
        total: Arc<AtomicU64>,
        done: Arc<AtomicBool>,
    ) {
        if done.load(AtomicOrdering::SeqCst) {
            return;
        }
        ctx.spawn(
            async move {
                Timer::after(Duration::from_millis(250)).await;
            },
            move |state, _, ctx| {
                if let Self::Downloading {
                    tag: cur_tag,
                    downloaded_bytes,
                    total_bytes,
                } = state
                {
                    if cur_tag == &tag {
                        *downloaded_bytes = progress.load(AtomicOrdering::SeqCst);
                        let t = total.load(AtomicOrdering::SeqCst);
                        *total_bytes = if t > 0 { Some(t) } else { None };
                        ctx.notify();
                    }
                }
                Self::schedule_progress_tick(ctx, tag, progress, total, done);
            },
        );
    }

    #[cfg(not(target_os = "macos"))]
    pub fn trigger_install(_ctx: &mut AppContext, _target: InstallableRelease) {
        // No-op outside macOS; the UI does not surface the install button
        // on other platforms.
    }
}

impl Default for GithubUpdateState {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity for GithubUpdateState {
    type Event = ();
}

impl SingletonEntity for GithubUpdateState {}

pub(crate) enum CheckResult {
    UpToDate,
    UpdateAvailable {
        tag: String,
        html_url: String,
        installable: Option<InstallableRelease>,
    },
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: Option<String>,
    #[serde(default)]
    body: String,
    #[serde(default)]
    published_at: Option<String>,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    assets: Vec<ReleaseAsset>,
}

/// Trimmed view of a release used by the in-app "What's New" panel.
/// Decoupled from `GithubRelease` so the public surface stays stable
/// even if the GitHub schema we deserialize evolves.
///
/// Note: the `/releases/latest` endpoint we hit deliberately skips
/// drafts and prereleases. If the fork ever publishes prereleases the
/// What's New panel will lag the actual release; switch to `/releases`
/// + first-non-draft if that ever ships.
#[derive(Clone, Debug)]
pub struct ReleaseNotes {
    pub tag: String,
    pub body: String,
    /// RFC 3339 timestamp from the GitHub Releases payload. Left as an
    /// `Option<String>` so the caller (autoupdate::changelog) owns parsing
    /// — keeps this module free of chrono.
    pub published_at: Option<String>,
}

/// Fetches the latest release notes from the fork's GitHub Releases.
///
/// Mirrors [`check_for_update`]'s transport layer (fresh `reqwest::Client`,
/// same UA / Accept / optional token) so we don't leak warp telemetry
/// headers to api.github.com.
pub async fn fetch_release_notes() -> Result<ReleaseNotes> {
    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .context("Failed to construct reqwest client")?;
    let mut request = client
        .get(REPO_API_URL)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::ACCEPT, ACCEPT_HEADER);
    if let Some(token) = resolve_github_token() {
        request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
    }
    let release: GithubRelease = request
        .send()
        .await
        .context("Failed to fetch latest GitHub release")?
        .error_for_status()
        .context("GitHub latest release request failed")?
        .json()
        .await
        .context("Failed to parse latest GitHub release response")?;

    if release.tag_name.is_empty() {
        bail!("GitHub release missing tag_name");
    }

    Ok(ReleaseNotes {
        tag: release.tag_name,
        body: release.body,
        published_at: release.published_at,
    })
}

#[derive(Clone, Serialize, Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

/// Picks the tarball + matching `.minisig` from a release's asset list.
/// Returns `None` (and the UI falls back to the browser link) if either
/// piece is missing — older releases predate the auto-update pipeline,
/// and a release with the tarball but no signature is treated identically:
/// without the signature we cannot verify integrity, so we refuse to
/// auto-install.
fn select_installable(tag: &str, assets: &[ReleaseAsset]) -> Option<InstallableRelease> {
    if !GithubUpdateState::install_supported() {
        return None;
    }
    // Filter on URL prefix as well as name suffix so a poisoned cache
    // can't aim the installer at an arbitrary host before signature
    // verification gets a chance to reject it.
    let tarball = assets.iter().find(|a| {
        a.name.ends_with(".tar.gz")
            && !a.name.ends_with(".tar.gz.minisig")
            && is_repo_release_download_url(&a.browser_download_url)
    })?;
    let sig = assets.iter().find(|a| {
        a.name == format!("{}.minisig", tarball.name)
            && is_repo_release_download_url(&a.browser_download_url)
    })?;
    Some(InstallableRelease {
        tag: tag.to_string(),
        asset_url: tarball.browser_download_url.clone(),
        sig_url: sig.browser_download_url.clone(),
    })
}

async fn check_for_update(force: bool) -> Result<CheckResult> {
    let cached = read_latest_release_cache();

    // Fast path: a fresh-enough cache short-circuits the network entirely on
    // auto-check. Manual `force` always falls through to do at least a
    // conditional GET so the user sees real freshness, not a cached "up to
    // date" from yesterday.
    if !force {
        if let Some(c) = cached.as_ref() {
            if now_secs().saturating_sub(c.fetched_at) < AUTO_CHECK_THROTTLE.as_secs() {
                return Ok(check_result_from_cache(c));
            }
        }
    }

    // Use a fresh `reqwest::Client` rather than `http_client::Client`: the
    // shared client injects Warp telemetry headers (`X-Warp-Client-ID`, OS
    // info, etc.) on every native request, which we must not leak to a
    // third-party host like api.github.com.
    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .context("Failed to construct reqwest client")?;
    let mut request = client
        .get(REPO_API_URL)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::ACCEPT, ACCEPT_HEADER);
    if let Some(token) = resolve_github_token() {
        request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
    }
    if let Some(etag) = cached.as_ref().and_then(|c| c.etag.clone()) {
        request = request.header(reqwest::header::IF_NONE_MATCH, etag);
    }

    let response = request
        .send()
        .await
        .context("Failed to fetch latest GitHub release")?;

    let status = response.status();

    // 304: server confirms cached payload still current. Bump fetched_at
    // (so the next check honors the throttle window) and reuse cache. This
    // is free under the rate limiter — 304 responses don't consume budget.
    if status == reqwest::StatusCode::NOT_MODIFIED {
        if let Some(mut c) = cached {
            c.fetched_at = now_secs();
            let result = check_result_from_cache(&c);
            if let Err(err) = write_latest_release_cache(&c) {
                log::debug!("write latest release cache failed: {err:#}");
            }
            return Ok(result);
        }
        // 304 with no cache should not happen (no etag was sent), but be
        // defensive: fall through to a fresh GET below by bailing here so
        // the caller treats it as transient.
        bail!("GitHub returned 304 with no local cache");
    }

    // 403 with unambiguous rate-limit headers: degrade to the tags API, which
    // sits on a separate budget for the unauthenticated case. Real forbidden
    // responses (bad credentials, repo access revoked, etc.) ship neither
    // `x-ratelimit-reset` nor `Retry-After` and must NOT be hidden behind
    // stale cache, so the pattern check is strict.
    if status == reqwest::StatusCode::FORBIDDEN {
        if let Some(pattern) = github_forbidden_rate_limit_pattern(response.headers()) {
            log::warn!("GitHub /releases/latest 403 matched rate-limit pattern: {pattern}");
            if let Some(mut c) = cached {
                log::warn!("reusing cached release while rate-limited");
                // Bump fetched_at so subsequent auto-checks honor the throttle
                // and don't keep retrying every spawn cycle while we're still
                // in the rate-limit window.
                c.fetched_at = now_secs();
                let result = check_result_from_cache(&c);
                if let Err(err) = write_latest_release_cache(&c) {
                    log::debug!("write latest release cache failed: {err:#}");
                }
                return Ok(result);
            }
            match check_via_tags_fallback(&client).await {
                Ok(cache) => {
                    let result = check_result_from_cache(&cache);
                    if let Err(err) = write_latest_release_cache(&cache) {
                        log::debug!("write latest release cache failed: {err:#}");
                    }
                    return Ok(result);
                }
                Err(err) => bail!(
                    "GitHub /releases/latest rate-limited and tags fallback failed: {err:#}"
                ),
            }
        }
        // Non-rate-limit 403 (auth failure, repo private, etc.) falls through
        // to error_for_status() below — surface it instead of hiding behind
        // stale cache.
    }

    let response = response
        .error_for_status()
        .context("GitHub latest release request failed")?;
    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let release: GithubRelease = response
        .json()
        .await
        .context("Failed to parse latest GitHub release response")?;

    if release.prerelease {
        log::info!("Latest GitHub release is a prerelease; treating as up to date");
        return Ok(CheckResult::UpToDate);
    }

    let tag = release.tag_name;
    if tag.is_empty() {
        bail!("GitHub release missing tag_name");
    }
    let html_url = release_html_url(&tag, release.html_url.as_deref());

    let cache = LatestReleaseCache {
        etag,
        fetched_at: now_secs(),
        tag: tag.clone(),
        html_url: html_url.clone(),
        body: release.body,
        published_at: release.published_at,
        prerelease: release.prerelease,
        assets: release.assets,
    };
    if let Err(err) = write_latest_release_cache(&cache) {
        log::debug!("write latest release cache failed: {err:#}");
    }
    Ok(check_result_from_cache(&cache))
}

/// Compare `cache.tag` to the locally-resolved app version and translate
/// into a `CheckResult`. Centralized so the fast-path (cache fresh), 304
/// path, and 200 path all agree on the comparison rule.
fn check_result_from_cache(cache: &LatestReleaseCache) -> CheckResult {
    if cache.prerelease {
        return CheckResult::UpToDate;
    }
    let tag = cache.tag.clone();
    // Sanitize cached html_url back to the canonical form. Stops a
    // tampered cache from surfacing an arbitrary link in the UI.
    let html_url = release_html_url(&tag, Some(&cache.html_url));
    let installable = select_installable(&tag, &cache.assets);

    let latest = match ParsedGithubVersion::parse(&tag) {
        Ok(v) => v,
        Err(err) => {
            log::warn!("Failed to parse cached release tag {tag}: {err:#}");
            return CheckResult::UpToDate;
        }
    };
    let current_version = ChannelState::app_version().or(option_env!("GIT_RELEASE_TAG"));
    match current_version {
        None => CheckResult::UpdateAvailable {
            tag,
            html_url,
            installable,
        },
        Some(current_tag) => match ParsedGithubVersion::parse(current_tag) {
            Ok(current) if latest.cmp_numeric(&current) == Ordering::Greater => {
                CheckResult::UpdateAvailable {
                    tag,
                    html_url,
                    installable,
                }
            }
            Ok(_) => CheckResult::UpToDate,
            Err(err) => {
                log::warn!("Failed to parse current version tag {current_tag}: {err:#}");
                CheckResult::UpdateAvailable {
                    tag,
                    html_url,
                    installable,
                }
            }
        },
    }
}

/// 403 fallback: list git refs (different rate budget than `/releases`),
/// pick the highest-version parseable tag, and synthesize a `CheckResult`.
/// `installable` is always `None` — we have no asset list — so the UI
/// falls back to the browser link, which is the safe behavior when the
/// API has refused to talk to us.
async fn check_via_tags_fallback(client: &reqwest::Client) -> Result<LatestReleaseCache> {
    let refs: Vec<GitRef> = github_get(client, REPO_TAGS_API)
        .await
        .context("tags fallback: list refs")?;
    let (tag, _latest) = refs
        .iter()
        .filter_map(|r| {
            let tag = r
                .ref_name
                .strip_prefix("refs/tags/")
                .unwrap_or(&r.ref_name)
                .to_string();
            ParsedGithubVersion::parse(&tag).ok().map(|p| (tag, p))
        })
        .max_by(|a, b| a.1.cmp_numeric(&b.1))
        .ok_or_else(|| anyhow!("tags fallback: no parseable release tag found"))?;

    Ok(LatestReleaseCache {
        etag: None,
        fetched_at: now_secs(),
        tag: tag.clone(),
        html_url: release_html_url(&tag, None),
        body: String::new(),
        published_at: None,
        prerelease: false,
        // No asset list available from this endpoint → installer falls
        // back to the browser link, which is the safe behavior when the
        // /releases endpoint has refused to talk to us.
        assets: Vec::new(),
    })
}

#[derive(Clone, Serialize, Deserialize)]
struct LatestReleaseCache {
    /// `ETag` from the most recent successful `/releases/latest` response.
    /// Sent back as `If-None-Match` so a no-change check returns 304 (free
    /// under the rate limiter).
    #[serde(default)]
    etag: Option<String>,
    fetched_at: u64,
    tag: String,
    html_url: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    published_at: Option<String>,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    assets: Vec<ReleaseAsset>,
}

fn latest_release_cache_path() -> std::path::PathBuf {
    paths::cache_dir().join(LATEST_RELEASE_CACHE_FILE)
}

fn read_latest_release_cache() -> Option<LatestReleaseCache> {
    read_capped_json(&latest_release_cache_path(), MAX_LATEST_RELEASE_CACHE_BYTES)
}

fn write_latest_release_cache(cache: &LatestReleaseCache) -> Result<()> {
    let path = latest_release_cache_path();
    let bytes = serde_json::to_vec(cache).context("serialize cache")?;
    if (bytes.len() as u64) > MAX_LATEST_RELEASE_CACHE_BYTES {
        bail!("latest release cache is too large");
    }
    write_json_atomically(&path, &bytes).context("write cache")?;
    Ok(())
}

/// Read JSON from `path`, refusing files above `max_bytes`. Returns `None`
/// for any failure mode (missing, oversized, malformed, symlink) — the
/// caller is expected to recover by re-fetching.
///
/// Refuses symlinks outright and enforces the byte cap on the actually-read
/// stream rather than `metadata.len()`, so a swap between stat-and-read
/// cannot smuggle in an oversized payload.
pub(crate) fn read_capped_json<T: for<'de> Deserialize<'de>>(
    path: &Path,
    max_bytes: u64,
) -> Option<T> {
    let file_type = std::fs::symlink_metadata(path).ok()?.file_type();
    if file_type.is_symlink() {
        log::warn!(
            "github_update: refusing symlink cache at {}",
            path.display()
        );
        return None;
    }

    let file = std::fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .ok()?;
    if (bytes.len() as u64) > max_bytes {
        log::warn!(
            "github_update: ignoring oversized cache at {} ({} bytes)",
            path.display(),
            bytes.len()
        );
        return None;
    }
    serde_json::from_slice(&bytes).ok()
}

/// Write `bytes` to `path` via tempfile + rename so a crash mid-write
/// never corrupts the previous good cache (plain `std::fs::write` can
/// leave a half-written file that subsequent reads silently discard,
/// erasing useful state).
pub(crate) fn write_json_atomically(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("cache path has no parent: {}", path.display()))?;
    std::fs::create_dir_all(parent).context("create cache dir")?;
    let mut tmp = tempfile::Builder::new()
        .prefix(".github_update.")
        .suffix(".tmp")
        .tempfile_in(parent)
        .context("create temp cache file")?;
    tmp.write_all(bytes).context("write temp cache file")?;
    tmp.as_file_mut().sync_all().context("sync temp cache file")?;
    tmp.persist(path)
        .map_err(|err| err.error)
        .with_context(|| format!("replace cache file {}", path.display()))?;
    Ok(())
}

/// Classify a 403 by header pattern. Returns the matched pattern label
/// (suitable for log output) only when GitHub's rate-limit shape is
/// unambiguous — `x-ratelimit-remaining: 0` accompanied by `x-ratelimit-reset`,
/// or a `Retry-After` header. Real auth-failure 403s ship neither of those,
/// so they fall through to `error_for_status()` rather than being silently
/// hidden behind a stale cache.
fn github_forbidden_rate_limit_pattern(
    headers: &reqwest::header::HeaderMap,
) -> Option<&'static str> {
    let remaining_zero = headers
        .get("x-ratelimit-remaining")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|remaining| remaining.trim() == "0");
    let has_reset = headers.contains_key("x-ratelimit-reset");
    let has_retry_after = headers.contains_key(reqwest::header::RETRY_AFTER);

    match (remaining_zero && has_reset, has_retry_after) {
        (true, true) => Some("x-ratelimit-remaining=0+reset+retry-after"),
        (true, false) => Some("x-ratelimit-remaining=0+reset"),
        (false, true) => Some("retry-after"),
        (false, false) => None,
    }
}

/// Canonicalize a release HTML URL. We accept the GitHub-supplied value
/// only when it points at this fork's release page; anything else (a
/// tampered cache, a redirect that snuck past, etc.) gets rebuilt from
/// the tag so the link the user clicks is always under our control.
fn release_html_url(tag: &str, candidate: Option<&str>) -> String {
    let expected_prefix = format!("{REPO_RELEASES_URL}/tag/");
    candidate
        .filter(|url| !url.is_empty() && url.starts_with(&expected_prefix))
        .map(str::to_string)
        .unwrap_or_else(|| format!("{REPO_RELEASES_URL}/tag/{tag}"))
}

fn is_repo_release_download_url(url: &str) -> bool {
    let expected_prefix = format!("{REPO_RELEASES_URL}/download/");
    url.starts_with(&expected_prefix)
}

/// Boot-time helper: synthesize a `CheckResult` from an existing on-disk
/// cache (no network). Returns `None` if no cache exists. Used by
/// [`auto_check`] to pre-seed `GithubUpdateState` so the menu badge /
/// version-row text reflect "update available" from the very first frame
/// instead of staying `Idle` until the 5s startup timer fires.
pub(crate) fn restore_state_from_cache() -> Option<CheckResult> {
    read_latest_release_cache()
        .as_ref()
        .map(check_result_from_cache)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedGithubVersion {
    major: u32,
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    channel: String,
    patch: u32,
}

impl ParsedGithubVersion {
    /// Accepts two tag shapes:
    ///   - `v<major>.<YYYY>.<MM>.<DD>.<HH>.<mm>.<channel>_<patch>`  (7-segment, CI)
    ///   - `v<major>.<YYYY>.<MM>.<DD>-<channel>.<patch>`            (5-segment, GitHub Releases)
    fn parse(tag: &str) -> Result<Self> {
        let body = tag
            .strip_prefix('v')
            .ok_or_else(|| anyhow!("Version tag must start with 'v'"))?;
        let parts: Vec<&str> = body.split('.').collect();

        match parts.len() {
            5 => {
                let (day, channel) = parts[3]
                    .split_once('-')
                    .ok_or_else(|| anyhow!("4th component must be DD-channel"))?;
                Ok(Self {
                    major: parts[0].parse().context("Invalid major version")?,
                    year: parts[1].parse().context("Invalid year")?,
                    month: parts[2].parse().context("Invalid month")?,
                    day: day.parse().context("Invalid day")?,
                    hour: 0,
                    minute: 0,
                    channel: channel.to_string(),
                    patch: parts[4].parse().context("Invalid patch")?,
                })
            }
            7 => {
                let (channel, patch) = parts[6]
                    .split_once('_')
                    .ok_or_else(|| anyhow!("7th component must be <channel>_<patch>"))?;
                Ok(Self {
                    major: parts[0].parse().context("Invalid major version")?,
                    year: parts[1].parse().context("Invalid year")?,
                    month: parts[2].parse().context("Invalid month")?,
                    day: parts[3].parse().context("Invalid day")?,
                    hour: parts[4].parse().context("Invalid hour")?,
                    minute: parts[5].parse().context("Invalid minute")?,
                    channel: channel.to_string(),
                    patch: patch.parse().context("Invalid patch")?,
                })
            }
            _ => bail!("Version tag must have 5 or 7 dot-separated components"),
        }
    }

    fn cmp_numeric(&self, other: &Self) -> Ordering {
        (
            self.major,
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.patch,
        )
            .cmp(&(
                other.major,
                other.year,
                other.month,
                other.day,
                other.hour,
                other.minute,
                other.patch,
            ))
    }
}

fn resolve_github_token() -> Option<String> {
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        if !token.is_empty() {
            return Some(token);
        }
    }
    read_gh_cli_token()
}

fn read_gh_cli_token() -> Option<String> {
    // gh CLI stores its config at `~/.config/gh/hosts.yml` on every platform
    // (XDG-style), not at `dirs::config_dir()/gh/...` — which on macOS resolves
    // to `~/Library/Application Support/gh/`, where gh never writes. Try the
    // XDG path first (covers the gh-on-macOS case), then $XDG_CONFIG_HOME if
    // set, then fall back to `dirs::config_dir()` for any platform where gh
    // does honor it.
    let mut candidates: Vec<std::path::PathBuf> = Vec::with_capacity(3);
    if let Some(home) = dirs::home_dir() {
        candidates.push(home.join(".config").join("gh").join("hosts.yml"));
    }
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            candidates.push(std::path::PathBuf::from(xdg).join("gh").join("hosts.yml"));
        }
    }
    if let Some(cfg) = dirs::config_dir() {
        candidates.push(cfg.join("gh").join("hosts.yml"));
    }

    for path in candidates {
        if let Some(token) = parse_gh_hosts_yaml(&path) {
            return Some(token);
        }
    }
    None
}

fn parse_gh_hosts_yaml(path: &std::path::Path) -> Option<String> {
    let contents = std::fs::read_to_string(path).ok()?;
    let hosts: serde_yaml::Value = serde_yaml::from_str(&contents).ok()?;
    let github = hosts.get("github.com")?;
    if let Some(token) = github.get("oauth_token").and_then(|v| v.as_str()) {
        if !token.is_empty() {
            return Some(token.to_string());
        }
    }
    let (_, first_user) = github.get("users")?.as_mapping()?.into_iter().next()?;
    first_user
        .get("oauth_token")?
        .as_str()
        .filter(|s| !s.is_empty())
        .map(String::from)
}

// =============================================================================
// SHA → tag reverse resolution (lets a retag fix a stale version without a
// rebuild or repackage). Triggered once at app startup; falls back silently to
// whatever ChannelState::app_version() already holds (plist or option_env).
// =============================================================================

#[derive(Deserialize)]
struct GitRef {
    #[serde(rename = "ref")]
    ref_name: String,
    object: GitRefObject,
}

#[derive(Deserialize)]
struct GitRefObject {
    sha: String,
    #[serde(rename = "type")]
    obj_type: String,
}

#[derive(Deserialize)]
struct GitTagObject {
    object: GitTagInner,
}

#[derive(Deserialize)]
struct GitTagInner {
    sha: String,
}

#[derive(Serialize, Deserialize)]
struct VersionCache {
    sha: String,
    tag: String,
    ts: u64,
}

/// Triggers a one-shot SHA→tag resolution against the fork's GitHub Releases.
/// Runs as a spawn on the GithubUpdateState entity (no state mutation; only
/// `ctx.notify()` on success so the version row re-renders). Pure
/// best-effort: any error (no SHA baked in, network down, rate limit, no
/// matching tag) leaves the existing app_version untouched.
/// Best-effort cleanup of the rollback `.previous` bundle left behind by a
/// successful auto-update swap. Reaching this code path means the new
/// version booted far enough to register singletons — i.e. the upgrade
/// looks healthy — so the rollback copy is no longer needed.
pub fn cleanup_previous_install() {
    #[cfg(target_os = "macos")]
    install::cleanup_previous();
}

pub fn trigger_app_version_resolve(ctx: &mut AppContext) {
    let Some(sha) = option_env!("GIT_COMMIT_SHA") else {
        return;
    };
    if sha.is_empty() {
        return;
    }
    let sha = sha.to_string();
    GithubUpdateState::handle(ctx).update(ctx, |_state, ctx| {
        let _ = ctx.spawn(
            async move { resolve_app_version_from_sha(&sha).await },
            |_state, result, ctx| match result {
                Ok(()) => ctx.notify(),
                Err(err) => {
                    log::debug!("github_update: app version resolve skipped: {err:#}");
                }
            },
        );
    });
}

async fn resolve_app_version_from_sha(sha: &str) -> Result<()> {
    if let Some(cache) = read_version_cache() {
        if cache.sha == sha && now_secs().saturating_sub(cache.ts) < VERSION_CACHE_TTL.as_secs() {
            apply_resolved_tag(&cache.tag);
            return Ok(());
        }
    }

    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .context("Failed to construct reqwest client")?;

    let refs: Vec<GitRef> = github_get(&client, REPO_TAGS_API)
        .await
        .context("Failed to list git refs")?;

    for git_ref in &refs {
        let commit_sha = match git_ref.object.obj_type.as_str() {
            "commit" => git_ref.object.sha.clone(),
            "tag" => match dereference_annotated_tag(&client, &git_ref.object.sha).await {
                Ok(s) => s,
                Err(err) => {
                    log::debug!("dereference annotated tag failed: {err:#}");
                    continue;
                }
            },
            _ => continue,
        };
        if commit_sha != sha {
            continue;
        }
        let tag = git_ref
            .ref_name
            .strip_prefix("refs/tags/")
            .unwrap_or(&git_ref.ref_name)
            .to_string();
        if ParsedGithubVersion::parse(&tag).is_err() {
            continue;
        }
        apply_resolved_tag(&tag);
        if let Err(err) = write_version_cache(&VersionCache {
            sha: sha.to_string(),
            tag,
            ts: now_secs(),
        }) {
            log::debug!("write version cache failed: {err:#}");
        }
        return Ok(());
    }

    bail!("no release-shaped tag points at {sha}");
}

async fn github_get<T: for<'de> Deserialize<'de>>(
    client: &reqwest::Client,
    url: &str,
) -> Result<T> {
    let mut request = client
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::ACCEPT, ACCEPT_HEADER);
    if let Some(token) = resolve_github_token() {
        request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"));
    }
    Ok(request
        .send()
        .await
        .context("send")?
        .error_for_status()
        .context("status")?
        .json()
        .await
        .context("json")?)
}

async fn dereference_annotated_tag(client: &reqwest::Client, tag_sha: &str) -> Result<String> {
    let url = format!("{REPO_GIT_TAGS_API}{tag_sha}");
    let obj: GitTagObject = github_get(client, &url).await?;
    Ok(obj.object.sha)
}

fn apply_resolved_tag(tag: &str) {
    let leaked: &'static str = Box::leak(tag.to_string().into_boxed_str());
    ChannelState::set_app_version(Some(leaked));
    log::info!("github_update: app_version resolved to {leaked} via SHA reverse lookup");
}

fn version_cache_path() -> std::path::PathBuf {
    paths::cache_dir().join(VERSION_CACHE_FILE)
}

fn read_version_cache() -> Option<VersionCache> {
    let bytes = std::fs::read(version_cache_path()).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn write_version_cache(cache: &VersionCache) -> Result<()> {
    let path = version_cache_path();
    let bytes = serde_json::to_vec(cache).context("serialize cache")?;
    write_json_atomically(&path, &bytes).context("write cache")?;
    Ok(())
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_7_segment() {
        let p = ParsedGithubVersion::parse("v0.2026.04.27.15.32.stable_03").unwrap();
        assert_eq!(p.major, 0);
        assert_eq!(p.year, 2026);
        assert_eq!(p.month, 4);
        assert_eq!(p.day, 27);
        assert_eq!(p.hour, 15);
        assert_eq!(p.minute, 32);
        assert_eq!(p.channel, "stable");
        assert_eq!(p.patch, 3);
    }

    #[test]
    fn parse_5_segment() {
        let p = ParsedGithubVersion::parse("v0.2026.05.02-cn.0").unwrap();
        assert_eq!(p.major, 0);
        assert_eq!(p.year, 2026);
        assert_eq!(p.month, 5);
        assert_eq!(p.day, 2);
        assert_eq!(p.hour, 0);
        assert_eq!(p.minute, 0);
        assert_eq!(p.channel, "cn");
        assert_eq!(p.patch, 0);
    }

    #[test]
    fn newer_patch_is_greater() {
        let a = ParsedGithubVersion::parse("v0.2026.04.27.15.32.stable_03").unwrap();
        let b = ParsedGithubVersion::parse("v0.2026.04.27.15.32.stable_05").unwrap();
        assert_eq!(b.cmp_numeric(&a), Ordering::Greater);
        assert_eq!(a.cmp_numeric(&b), Ordering::Less);
        assert_eq!(a.cmp_numeric(&a.clone()), Ordering::Equal);
    }

    #[test]
    fn newer_date_5_segment() {
        let a = ParsedGithubVersion::parse("v0.2026.05.01-cn.0").unwrap();
        let b = ParsedGithubVersion::parse("v0.2026.05.02-cn.0").unwrap();
        assert_eq!(b.cmp_numeric(&a), Ordering::Greater);
    }

    #[test]
    fn newer_date_is_greater() {
        let a = ParsedGithubVersion::parse("v0.2026.04.27.15.32.stable_03").unwrap();
        let b = ParsedGithubVersion::parse("v0.2026.04.28.10.00.stable_01").unwrap();
        assert_eq!(b.cmp_numeric(&a), Ordering::Greater);
    }

    #[test]
    fn missing_prefix_or_components_fails() {
        assert!(ParsedGithubVersion::parse("0.2026.04.27.15.32.stable_03").is_err());
        assert!(ParsedGithubVersion::parse("v0.2026.04.27.15.stable_03").is_err());
        assert!(ParsedGithubVersion::parse("v0.2026.04.27.15.32.stable").is_err());
        assert!(ParsedGithubVersion::parse("v0.2026.05.02cn.0").is_err());
    }
}
