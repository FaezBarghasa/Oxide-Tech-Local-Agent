use memory::{EphemeralMemory, TemporalCommit, TemporalGitMemory};

#[test]
fn test_ephemeral_memory_ring_buffer_and_editor_state() {
    let mem = EphemeralMemory::new(3);

    // Record 4 terminal commands (exceeding capacity 3)
    mem.record_terminal_output("sess_1", "cargo check", "Finished dev", Some(0));
    mem.record_terminal_output("sess_1", "cargo test", "running 2 tests", Some(0));
    mem.record_terminal_output("sess_1", "git status", "On branch main", Some(0));
    mem.record_terminal_output("sess_1", "pnpm build", "built in 1.8s", Some(0));

    let logs = mem.get_recent_terminal_logs(10);
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].command, "pnpm build"); // most recent first

    // Test editor buffer tracking
    mem.update_editor_buffer("src/main.rs", 42, 10, None, true);
    let open_bufs = mem.get_open_buffers();
    assert_eq!(open_bufs.len(), 1);
    assert_eq!(open_bufs[0].file_path, "src/main.rs");
    assert_eq!(open_bufs[0].cursor_line, 42);
    assert!(open_bufs[0].is_dirty);

    // Test stack trace push
    mem.push_stack_trace(
        "Panic",
        "assertion failed",
        vec!["main.rs:45".to_string(), "lib.rs:12".to_string()],
    );
    let trace = mem.get_latest_stack_trace();
    assert!(trace.is_some());
    assert_eq!(trace.unwrap().error_type, "Panic");
}

#[test]
fn test_temporal_git_churn_and_co_change() {
    let mut git_mem = TemporalGitMemory::new();

    git_mem.record_commit(TemporalCommit {
        hash: "c1".to_string(),
        author: "Dev <dev@oxide.tech>".to_string(),
        message: "feat: update auth and token handling".to_string(),
        timestamp: "2026-08-20T10:00:00Z".to_string(),
        files_changed: vec!["src/auth.rs".to_string(), "src/token.rs".to_string()],
    });

    git_mem.record_commit(TemporalCommit {
        hash: "c2".to_string(),
        author: "Dev <dev@oxide.tech>".to_string(),
        message: "fix: token expiration check".to_string(),
        timestamp: "2026-08-21T11:00:00Z".to_string(),
        files_changed: vec![
            "src/auth.rs".to_string(),
            "src/token.rs".to_string(),
            "src/db.rs".to_string(),
        ],
    });

    let auth_metrics = git_mem.get_churn_metrics("src/auth.rs");
    assert_eq!(auth_metrics.modification_count, 2);
    assert_eq!(auth_metrics.churn_risk_score, 1.0);

    // token.rs co-changed with auth.rs in 100% (2/2) of commits
    assert!(!auth_metrics.top_co_changed_files.is_empty());
    assert_eq!(auth_metrics.top_co_changed_files[0].0, "src/token.rs");
    assert_eq!(auth_metrics.top_co_changed_files[0].1, 1.0);
}
