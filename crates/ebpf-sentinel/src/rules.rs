use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyscallCategory {
    NetworkConnect,
    NetworkBind,
    FileWrite,
    ProcessSpawn,
    PrivilegeEscalation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalRule {
    pub category: SyscallCategory,
    pub path_whitelist: Vec<String>,
    pub disallowed_ports: Vec<u16>,
    pub allow_subprocesses: bool,
}

impl Default for ConstitutionalRule {
    fn default() -> Self {
        Self {
            category: SyscallCategory::FileWrite,
            path_whitelist: vec!["/tmp/".to_string(), "./target/".to_string()],
            disallowed_ports: vec![22, 23, 8080],
            allow_subprocesses: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleViolation {
    pub pid: u32,
    pub syscall_name: String,
    pub reason: String,
    pub timestamp_epoch_ms: u64,
}
