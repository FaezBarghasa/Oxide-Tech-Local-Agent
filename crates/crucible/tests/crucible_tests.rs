#[cfg(test)]
mod tests {
    use crucible::{MctsDecisionEngine, WorkspaceSnapshot};

    #[test]
    fn test_zero_copy_roundtrip() {
        let mut snapshot = WorkspaceSnapshot::new("task-42", 1);
        snapshot
            .registers
            .push(("reg0".to_string(), "0xCAFE".to_string()));
        snapshot
            .file_digests
            .push(("src/lib.rs".to_string(), [1u8; 32]));

        let bytes = snapshot.archive_to_bytes().expect("archive must succeed");
        let restored =
            WorkspaceSnapshot::from_archived_bytes(&bytes).expect("restore must succeed");

        assert_eq!(snapshot, restored);
    }

    #[test]
    fn test_mcts_candidate_evaluation() {
        let engine = MctsDecisionEngine::default();
        let snapshot = WorkspaceSnapshot::new("task-42", 1);
        let candidates = vec![
            "cargo check --workspace".to_string(),
            "rm -rf /tmp/test".to_string(),
            "cat src/main.rs".to_string(),
        ];

        let chosen = engine.evaluate_candidates(&snapshot, &candidates, |_st, action| {
            if action.starts_with("cargo check") {
                5.0
            } else if action.starts_with("rm") {
                -10.0
            } else {
                1.0
            }
        });

        assert!(chosen.is_some());
        let (action, score) = chosen.unwrap();
        assert_eq!(action, "cargo check --workspace");
        assert_eq!(score, 5.0);
    }
}
