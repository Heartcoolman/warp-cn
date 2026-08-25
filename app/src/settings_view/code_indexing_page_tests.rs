use remote_server::codebase_index_proto::{RemoteCodebaseIndexState, RemoteCodebaseIndexStatus};

use super::*;

fn remote_status_with_failure(failure_message: Option<&str>) -> RemoteCodebaseIndexStatus {
    RemoteCodebaseIndexStatus {
        repo_path: "/workspaces/repo".to_string(),
        state: RemoteCodebaseIndexState::Unavailable,
        last_updated_epoch_millis: Some(1),
        progress_completed: None,
        progress_total: None,
        failure_message: failure_message.map(ToOwned::to_owned),
        root_hash: None,
    }
}

#[test]
fn remote_index_limit_failure_is_detected_from_status_message() {
    let status = remote_status_with_failure(Some(
        "Cannot index remote codebase because the maximum number of codebase indexes has been reached.",
    ));

    assert!(remote_codebase_index_limit_reached(&status));
}

#[test]
fn other_unavailable_failures_are_not_index_limit_failures() {
    let status = remote_status_with_failure(Some(
        "Cannot index remote codebase because indexing did not start.",
    ));

    assert!(!remote_codebase_index_limit_reached(&status));
}

// ── Fork: i18n / stats-bar / error-reason unit tests (ported from old code_page.rs) ──

#[cfg(test)]
mod i18n_tests {
    use chrono::Duration as ChronoDuration;
    use serial_test::serial;

    use super::*;

    fn ensure_en() {
        let _ = warp_i18n::init(warp_i18n::Locale::En);
        warp_i18n::set_locale(warp_i18n::Locale::En);
    }

    fn workspace_with(
        synced_at: Option<DateTime<Utc>>,
        queried_ts: Option<DateTime<Utc>>,
        query_count: u32,
    ) -> WorkspaceMetadata {
        WorkspaceMetadata {
            path: PathBuf::from("/tmp/test-repo"),
            file_count: Some(123),
            fragment_count: Some(456),
            index_bytes: Some(2 * 1024),
            synced_at,
            queried_ts,
            query_count,
            ..Default::default()
        }
    }

    #[test]
    fn truncate_error_reason_passes_through_short_text() {
        assert_eq!(truncate_error_reason("oops"), "oops");
    }

    #[test]
    fn truncate_error_reason_takes_first_line_only() {
        assert_eq!(
            truncate_error_reason("first failure\nsecond line\nthird"),
            "first failure"
        );
    }

    #[test]
    fn truncate_error_reason_caps_long_messages_with_ellipsis() {
        let long = "a".repeat(ERROR_REASON_MAX_LEN + 50);
        let out = truncate_error_reason(&long);
        assert!(out.ends_with('…'));
        assert_eq!(out.chars().count(), ERROR_REASON_MAX_LEN + 1);
    }

    #[test]
    fn truncate_error_reason_handles_empty_and_blank() {
        assert_eq!(truncate_error_reason(""), "(no error message)");
        assert_eq!(truncate_error_reason("   \n  "), "(no error message)");
    }

    #[test]
    #[serial]
    fn humanize_bytes_label_picks_correct_unit() {
        ensure_en();
        assert!(humanize_bytes_label(0).contains("0 B"));
        assert!(humanize_bytes_label(512).contains("512 B"));
        assert!(humanize_bytes_label(1024).contains("1.0 KB"));
        assert!(humanize_bytes_label(1024 * 1024).contains("1.0 MB"));
        assert!(humanize_bytes_label(1024u64 * 1024 * 1024).contains("1.0 GB"));
    }

    #[test]
    #[serial]
    fn relative_time_label_walks_through_buckets() {
        ensure_en();
        let now = Utc::now();
        assert!(relative_time_label(now - ChronoDuration::seconds(30)).contains("just now"));
        assert!(relative_time_label(now - ChronoDuration::minutes(5)).contains("5"));
        assert!(relative_time_label(now - ChronoDuration::hours(2)).contains("2"));
        assert!(relative_time_label(now - ChronoDuration::days(3)).contains("3"));
    }

    #[test]
    #[serial]
    fn build_index_stats_line_returns_none_when_never_synced() {
        ensure_en();
        let ws = WorkspaceMetadata {
            path: PathBuf::from("/tmp/empty"),
            ..Default::default()
        };
        assert_eq!(build_index_stats_line(&ws, None), None);
    }

    #[test]
    #[serial]
    fn build_index_stats_line_includes_all_present_fields() {
        ensure_en();
        let ws = workspace_with(Some(Utc::now()), Some(Utc::now()), 3);
        let line = build_index_stats_line(&ws, Some(&CodebaseIndexFinishedStatus::Completed))
            .expect("expected stats line");
        assert!(line.contains("123"), "missing file count: {line}");
        assert!(line.contains("456"), "missing fragment count: {line}");
        assert!(line.contains("KB"), "missing bytes label: {line}");
        assert!(line.contains("synced"), "missing synced label: {line}");
        assert!(line.contains("queried"), "missing queried label: {line}");
        assert!(line.contains(" · "), "missing separator: {line}");
    }

    #[test]
    #[serial]
    fn build_index_stats_line_skips_query_section_when_count_zero() {
        ensure_en();
        let ws = workspace_with(Some(Utc::now()), None, 0);
        let line = build_index_stats_line(&ws, Some(&CodebaseIndexFinishedStatus::Completed))
            .expect("stats line");
        assert!(!line.contains("queried"), "should not show queried: {line}");
    }

    #[test]
    #[serial]
    fn build_index_stats_line_replaces_with_error_when_failed() {
        ensure_en();
        let ws = workspace_with(Some(Utc::now()), None, 0);
        let err = CodebaseIndexFinishedStatus::Failed(CodebaseIndexingError::ExceededMaxFileLimit);
        let line = build_index_stats_line(&ws, Some(&err)).expect("error line");
        // Must NOT include the stats fields when failed.
        assert!(
            !line.contains("123"),
            "stats leaked into error line: {line}"
        );
        assert!(
            !line.contains(" · "),
            "separator leaked into error line: {line}"
        );
    }

    #[test]
    #[serial]
    fn error_reason_text_covers_all_variants() {
        ensure_en();
        assert!(!error_reason_text(&CodebaseIndexingError::BuildTreeError).is_empty());
        assert!(!error_reason_text(&CodebaseIndexingError::ExceededMaxFileLimit).is_empty());
        assert!(!error_reason_text(&CodebaseIndexingError::MaxDepthExceeded).is_empty());
        assert!(
            !error_reason_text(&CodebaseIndexingError::FailedToGenerateEmbeddings(vec![]))
                .is_empty()
        );
        assert!(
            !error_reason_text(&CodebaseIndexingError::FailedToSyncIntermediateNodes(
                vec![]
            ))
            .is_empty()
        );
        let other_msg =
            error_reason_text(&CodebaseIndexingError::Other(anyhow::anyhow!("disk full")));
        assert!(other_msg.contains("disk full"), "other reason: {other_msg}");
    }
}

#[test]
fn current_team_disabling_indexing_uses_generic_tooltip_text() {
    assert_eq!(
        codebase_indexing_disabled_admin_text(Some("Team A"), true),
        warp_i18n::t!("settings-code-indexing-disabled-admin")
    );
}

#[test]
fn other_team_disabling_indexing_names_that_team() {
    assert_eq!(
        codebase_indexing_disabled_admin_text(Some("Team A"), false),
        warp_i18n::t!(
            "settings-code-indexing-disabled-admin-team",
            team = "Team A"
        )
    );
}
