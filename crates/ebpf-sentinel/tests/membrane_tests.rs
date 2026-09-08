#[cfg(test)]
mod tests {
    use ebpf_sentinel::{ConstitutionalMembrane, SyscallCategory};

    #[tokio::test]
    async fn test_constitutional_membrane_file_whitelist() {
        let membrane = ConstitutionalMembrane::new();
        
        // Allowed path
        let res_allowed = membrane.validate_action(1234, &SyscallCategory::FileWrite, "/tmp/sandbox_test.rs").await;
        assert!(res_allowed.is_ok());

        // Forbidden path
        let res_denied = membrane.validate_action(1234, &SyscallCategory::FileWrite, "/etc/shadow").await;
        assert!(res_denied.is_err());
        let violation = res_denied.unwrap_err();
        assert_eq!(violation.pid, 1234);
        assert!(violation.reason.contains("not in the constitutional whitelist"));

        let violations = membrane.get_violations().await;
        assert_eq!(violations.len(), 1);
    }
}
