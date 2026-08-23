use serde::{Deserialize, Serialize};
use tracing::info;
use crate::{execute_in_sandbox, ExecutionResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub key_path: Option<String>,
}

pub struct RemoteSshManager;

impl RemoteSshManager {
    /// Execute a command on a remote host via SSH
    pub async fn execute(
        config: &SshConfig,
        command: &str,
        work_dir: &str,
    ) -> Result<ExecutionResult, String> {
        let port_str = config.port.to_string();
        let target = format!("{}@{}", config.user, config.host);

        let mut args = vec![
            "ssh",
            "-p",
            &port_str,
            "-o",
            "StrictHostKeyChecking=no",
            "-o",
            "BatchMode=yes",
            "-o",
            "ConnectTimeout=10",
        ];

        let key_flag = "-i";
        if let Some(ref key) = config.key_path {
            args.push(key_flag);
            args.push(key.as_str());
        }

        args.push(&target);
        args.push(command);

        info!("Executing remote SSH on {}: {}", target, command);
        execute_in_sandbox(&args, work_dir)
            .await
            .map_err(|e| format!("Remote SSH command failed: {}", e))
    }

    /// Copy a file to remote target via SCP
    pub async fn copy_to_remote(
        config: &SshConfig,
        local_path: &str,
        remote_path: &str,
        work_dir: &str,
    ) -> Result<ExecutionResult, String> {
        let port_str = config.port.to_string();
        let dest = format!("{}@{}:{}", config.user, config.host, remote_path);

        let mut args = vec![
            "scp",
            "-P",
            &port_str,
            "-o",
            "StrictHostKeyChecking=no",
            "-o",
            "BatchMode=yes",
        ];

        let key_flag = "-i";
        if let Some(ref key) = config.key_path {
            args.push(key_flag);
            args.push(key.as_str());
        }

        args.push(local_path);
        args.push(&dest);

        info!("Copying artifact via SCP to {}", dest);
        execute_in_sandbox(&args, work_dir)
            .await
            .map_err(|e| format!("SCP file transfer failed: {}", e))
    }
}
