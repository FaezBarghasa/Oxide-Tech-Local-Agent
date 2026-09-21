use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::mpsc;

const MAX_QUEUE_CAPACITY: usize = 1024;
const MAX_LOG_SIZE_BYTES: u64 = 8 * 1024 * 1024; // 8 MiB size limit before rotation

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRecord {
    pub level: String,
    pub target: String,
    pub message: String,
    pub timestamp_ms: i64,
}

/// Bounded, non-blocking telemetry outbox that drops records on full queue without blocking caller
pub struct BoundedDiagnosticOutbox {
    tx: mpsc::Sender<DiagnosticRecord>,
    dropped_count: Arc<AtomicUsize>,
    log_path: PathBuf,
}

impl BoundedDiagnosticOutbox {
    pub fn new(log_path: impl AsRef<Path>) -> Self {
        let (tx, mut rx) = mpsc::channel::<DiagnosticRecord>(MAX_QUEUE_CAPACITY);
        let dropped_count = Arc::new(AtomicUsize::new(0));
        let path = log_path.as_ref().to_path_buf();

        let path_clone = path.clone();
        tokio::spawn(async move {
            let _ = std::fs::create_dir_all(path_clone.parent().unwrap_or_else(|| Path::new(".")));
            while let Some(record) = rx.recv().await {
                if let Ok(data) = serde_json::to_vec(&record) {
                    // Check file size rotation
                    if let Ok(meta) = std::fs::metadata(&path_clone)
                        && meta.len() > MAX_LOG_SIZE_BYTES
                    {
                        let rotated = path_clone.with_extension("jsonl.1");
                        let _ = std::fs::rename(&path_clone, rotated);
                    }

                    if let Ok(mut file) = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&path_clone)
                    {
                        let _ = file.write_all(&data);
                        let _ = file.write_all(b"\n");
                    }
                }
            }
        });

        Self {
            tx,
            dropped_count,
            log_path: path,
        }
    }

    /// Non-blocking send: if queue is full, increments dropped_count and returns immediately
    pub fn emit(&self, level: &str, target: &str, message: &str) -> bool {
        let record = DiagnosticRecord {
            level: level.to_string(),
            target: target.to_string(),
            message: message.to_string(),
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        match self.tx.try_send(record) {
            Ok(_) => true,
            Err(_) => {
                self.dropped_count.fetch_add(1, Ordering::Relaxed);
                false
            }
        }
    }

    pub fn dropped_records_count(&self) -> usize {
        self.dropped_count.load(Ordering::Relaxed)
    }

    pub fn log_file_path(&self) -> &Path {
        &self.log_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_bounded_diagnostic_outbox() {
        let temp = tempdir().unwrap();
        let log = temp.path().join("outbox.jsonl");
        let outbox = BoundedDiagnosticOutbox::new(&log);

        assert!(outbox.emit("INFO", "test", "Hello diagnostics"));
        assert_eq!(outbox.dropped_records_count(), 0);
    }
}
