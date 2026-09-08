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
            if &rule.category == category {
                if *category == SyscallCategory::FileWrite {
                    let allowed = rule
                        .path_whitelist
                        .iter()
                        .any(|prefix| target.starts_with(prefix));
                    if !allowed {
                        let violation = RuleViolation {
                            pid,
                            syscall_name: "openat/write".to_string(),
                            reason: format!(
                                "Path '{target}' is not in the constitutional whitelist"
                            ),
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
        }

        Ok(())
    }

    pub async fn get_violations(&self) -> Vec<RuleViolation> {
        let lock = self.violations.read().await;
        lock.clone()
    }
}
