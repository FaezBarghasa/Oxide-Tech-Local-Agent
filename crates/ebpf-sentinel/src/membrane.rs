use crate::rules::{ConstitutionalRule, RuleViolation, SyscallCategory};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

pub struct ConstitutionalMembrane {
    rules: Arc<RwLock<Vec<ConstitutionalRule>>>,
    violations: Arc<RwLock<Vec<RuleViolation>>>,
}

impl Default for ConstitutionalMembrane {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstitutionalMembrane {
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(vec![ConstitutionalRule::default()])),
            violations: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add an active constitutional rule
    pub async fn register_rule(&self, rule: ConstitutionalRule) {
        let mut lock = self.rules.write().await;
        lock.push(rule);
    }

    /// Intercept and validate a process action against kernel-level safety rules
    pub async fn validate_action(
        &self,
        pid: u32,
        category: &SyscallCategory,
        target: &str,
    ) -> Result<(), RuleViolation> {
        let rules = self.rules.read().await;

        for rule in rules.iter() {
            if &rule.category == category && *category == SyscallCategory::FileWrite {
                let allowed = rule
                    .path_whitelist
                    .iter()
                    .any(|prefix| target.starts_with(prefix));
                if !allowed {
                    let violation = RuleViolation {
                        pid,
                        syscall_name: "openat/write".to_string(),
                        reason: format!("Path '{target}' is not in the constitutional whitelist"),
                        timestamp_epoch_ms: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64,
                    };

                    let mut v_lock = self.violations.write().await;
                    v_lock.push(violation.clone());
                    error!(target: "ebpf_sentinel", "Constitutional violation detected: {:?}", violation);
                    return Err(violation);
                }
            }
        }

        Ok(())
    }

    pub async fn get_violations(&self) -> Vec<RuleViolation> {
        let lock = self.violations.read().await;
        lock.clone()
    }

    /// Apply POSIX resource limits (memory and open file descriptors) to the calling process.
    #[cfg(target_os = "linux")]
    pub fn apply_sandboxed_rlimits(
        max_address_space_bytes: u64,
        max_open_fds: u64,
    ) -> Result<(), String> {
        unsafe {
            // Set memory address space limit
            if max_address_space_bytes > 0 {
                let rlim_as = libc::rlimit {
                    rlim_cur: max_address_space_bytes as libc::rlim_t,
                    rlim_max: max_address_space_bytes as libc::rlim_t,
                };
                if libc::setrlimit(libc::RLIMIT_AS, &rlim_as) != 0 {
                    return Err(format!(
                        "Failed to set RLIMIT_AS: errno {}",
                        *libc::__errno_location()
                    ));
                }
            }

            // Set file descriptor limit
            if max_open_fds > 0 {
                let rlim_nofile = libc::rlimit {
                    rlim_cur: max_open_fds as libc::rlim_t,
                    rlim_max: max_open_fds as libc::rlim_t,
                };
                if libc::setrlimit(libc::RLIMIT_NOFILE, &rlim_nofile) != 0 {
                    return Err(format!(
                        "Failed to set RLIMIT_NOFILE: errno {}",
                        *libc::__errno_location()
                    ));
                }
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn apply_sandboxed_rlimits(
        _max_address_space_bytes: u64,
        _max_open_fds: u64,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_membrane_whitelist_validation() {
        let membrane = ConstitutionalMembrane::new();

        // Allowed path
        let allowed = membrane
            .validate_action(1234, &SyscallCategory::FileWrite, "/tmp/oxide_test.txt")
            .await;
        assert!(allowed.is_ok());

        // Disallowed path
        let disallowed = membrane
            .validate_action(1234, &SyscallCategory::FileWrite, "/etc/shadow")
            .await;
        assert!(disallowed.is_err());

        let violations = membrane.get_violations().await;
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].pid, 1234);
    }
}
