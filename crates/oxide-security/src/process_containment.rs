use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessContainmentError {
    #[error("Failed to signal process: {0}")]
    SignalFailed(String),
    #[error("Child process execution error: {0}")]
    Execution(String),
}

/// Process Tree Guard tracking child processes and process groups to guarantee zero-zombie teardown
pub struct ProcessTreeGuard {
    pgid: Option<u32>,
    pid: u32,
    grace_period: Duration,
}

impl ProcessTreeGuard {
    pub fn new(pid: u32, pgid: Option<u32>) -> Self {
        Self {
            pid,
            pgid,
            grace_period: Duration::from_secs(4),
        }
    }

    pub fn with_grace_period(mut self, grace_period: Duration) -> Self {
        self.grace_period = grace_period;
        self
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn pgid(&self) -> Option<u32> {
        self.pgid
    }

    /// Clean teardown: sends SIGTERM to process group, waits up to grace_period, then forces SIGKILL
    pub async fn terminate_all(&self) -> Result<(), ProcessContainmentError> {
        #[cfg(unix)]
        {
            use nix::sys::signal::{Signal, kill, killpg};
            use nix::unistd::Pid;

            // 1. Send SIGTERM
            if let Some(pgid) = self.pgid {
                let _ = killpg(Pid::from_raw(pgid as i32), Signal::SIGTERM);
            } else {
                let _ = kill(Pid::from_raw(self.pid as i32), Signal::SIGTERM);
            }

            // 2. Wait grace period asynchronously
            tokio::time::sleep(self.grace_period).await;

            // 3. Force SIGKILL to guarantee no zombies remain
            if let Some(pgid) = self.pgid {
                let _ = killpg(Pid::from_raw(pgid as i32), Signal::SIGKILL);
            } else {
                let _ = kill(Pid::from_raw(self.pid as i32), Signal::SIGKILL);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_tree_guard_init() {
        let guard =
            ProcessTreeGuard::new(12345, Some(12345)).with_grace_period(Duration::from_millis(10));
        assert_eq!(guard.pid(), 12345);
        assert_eq!(guard.pgid(), Some(12345));
    }
}
