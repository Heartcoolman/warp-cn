use std::iter;
use std::sync::Arc;

use anyhow::Result;
use channel_versions::{Changelog, ChannelVersions, MarkdownSection};
use chrono::DateTime;
use rand::distributions::Alphanumeric;
use rand::{Rng as _, thread_rng};
use warp_errors::report_error;

use super::channel_versions::fetch_channel_versions;
use super::release_assets_directory_url;
use crate::channel::{Channel, ChannelState};
use crate::github_update;
use crate::server::server_api::ServerApi;

pub async fn get_current_changelog(server_api: Arc<ServerApi>) -> Result<Option<Changelog>> {
    let channel = ChannelState::channel();

    // warp-cn fork: the `Channel::Oss` arm of the upstream pipeline always
    // returns `None` (see the match below), which surfaces as "无法获取最新更新日志"
    // in the What's New panel. Short-circuit to GitHub Releases instead so
    // the panel shows the fork's own release notes.
    if channel == Channel::Oss {
        return fetch_changelog_from_github_releases().await;
    }

    let rand: String = {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(7)
            .collect()
    };

    if should_fetch_changelog_json(channel) {
        log::info!("Attempting to fetch changelog.json");
        match fetch_current_changelog(server_api.http_client(), rand.as_str()).await {
            changelog_result @ Ok(_) => {
                return changelog_result.map(Option::Some);
            }
            Err(error) => {
                report_error!(error.context("Failed to fetch changelog.json"))
            }
        };
    }

    let versions: ChannelVersions =
        fetch_channel_versions(rand.as_str(), server_api, true, false).await?;

    let res = versions.changelogs.and_then(|changelogs| {
        match channel {
            Channel::Stable => Some(changelogs.stable),
            Channel::Preview => Some(changelogs.preview),
            Channel::Dev | Channel::Local => Some(changelogs.dev),
            // Integration tests and the open-source build don't support autoupdate.
            Channel::Integration | Channel::Oss => None,
        }
        .and_then(|versions| {
            ChannelState::app_version()
                .and_then(|running_version| versions.get(running_version))
                .cloned()
        })
    });
    Ok(res)
}

/// Fetches the changelog for the running release bundle, using the given http
/// client and cache-busting nonce.
async fn fetch_current_changelog(client: &http_client::Client, nonce: &str) -> Result<Changelog> {
    let app_version = ChannelState::app_version().unwrap_or_default();
    let url = format!(
        "{}?r={}",
        changelog_url(ChannelState::channel(), app_version),
        nonce
    );
    let res = client.get(url.as_str()).send().await?;
    let changelog: Changelog = res.json().await?;
    log::info!("Received changelog.json for {app_version}");
    Ok(changelog)
}

/// Returns the URL to the changelog for the given version of this release
/// bundle.
fn changelog_url(channel: Channel, version: &str) -> String {
    format!(
        "{}/changelog.json",
        release_assets_directory_url(channel, version)
    )
}

/// Returns whether the app should fetch changelog.json for the current
/// build (true), or use the changelog information embedded in
/// channel_versions.json (false).
pub fn should_fetch_changelog_json(channel: Channel) -> bool {
    channel == Channel::Dev
}

/// warp-cn changelog source: pulls the latest release body (markdown) from
/// the fork's GitHub Releases and adapts it into the existing
/// [`Changelog`] shape, so the same `ChangelogModel` / view path renders it
/// without further code changes.
async fn fetch_changelog_from_github_releases() -> Result<Option<Changelog>> {
    let notes = match github_update::fetch_release_notes().await {
        Ok(notes) => notes,
        Err(err) => {
            // Treat a fetch failure as "no changelog" rather than an error,
            // so the panel falls back to its existing empty/error state
            // instead of crashing the model.
            log::warn!("warp-cn: GitHub release notes fetch failed: {err:#}");
            return Ok(None);
        }
    };

    // Use the Unix epoch as a deterministic sentinel when the GitHub payload
    // omits `published_at` or it fails to parse. We deliberately avoid
    // `Utc::now()` here: the panel displays this date, and "today" would
    // shift each time the user reopens the panel — confusing for a release
    // that's actually weeks old. `/releases/latest` almost always carries
    // `published_at`, so this fallback is rare in practice.
    let date = notes
        .published_at
        .as_deref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .unwrap_or_else(|| {
            DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z")
                .expect("hardcoded RFC3339 epoch must parse")
        });

    // Pass an empty `Vec` when the release body is missing so
    // `ChangelogModel::maybe_add_changelog_sections` injects its existing
    // "No notable changes this release" fallback — keeps the empty-state
    // wording consistent across channels rather than echoing the same
    // string here.
    let body = notes.body.trim();
    let markdown_sections = if body.is_empty() {
        Vec::new()
    } else {
        vec![MarkdownSection {
            title: "New features".to_string(),
            markdown: body.to_string(),
        }]
    };

    log::info!(
        "warp-cn: loaded changelog from GitHub release {}",
        notes.tag
    );

    Ok(Some(Changelog {
        date,
        sections: Vec::new(),
        markdown_sections,
        image_url: None,
        oz_updates: Vec::new(),
        tui_updates: Vec::new(),
    }))
}
