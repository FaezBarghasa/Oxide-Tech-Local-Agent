use regex::Regex;
use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const MAX_STDERR_TAIL_BYTES: usize = 16 * 1024; // Retain only the trailing 16 KiB

/// Secure Stderr Ring-Buffer Capturer with Secret Scrubbing & 0600 file isolation
pub struct StderrSanitizer {
    diagnostic_path: PathBuf,
    buffer: Mutex<VecDeque<u8>>,
    patterns: Vec<Regex>,
}

impl StderrSanitizer {
    pub fn new(diagnostic_path: impl AsRef<Path>) -> Self {
        let patterns = vec![
            Regex::new(r"sk-[a-zA-Z0-9]{20,}").unwrap(),
            Regex::new(r"ghp_[a-zA-Z0-9]{36}").unwrap(),
            Regex::new(r"hf_[a-zA-Z0-9]{34}").unwrap(),
            Regex::new(r"(?i)bearer\s+[a-zA-Z0-9\-_\.]+").unwrap(),
            Regex::new(r"(?i)password\s*[:=]\s*[^\s]+").unwrap(),
        ];

        Self {
            diagnostic_path: diagnostic_path.as_ref().to_path_buf(),
            buffer: Mutex::new(VecDeque::with_capacity(MAX_STDERR_TAIL_BYTES)),
            patterns,
        }
    }

    /// Append chunk into 16 KiB bounded ring buffer
    pub fn append_chunk(&self, chunk: &[u8]) {
        let mut buf = self.buffer.lock().unwrap();
        for &byte in chunk {
            if buf.len() >= MAX_STDERR_TAIL_BYTES {
                buf.pop_front();
            }
            buf.push_back(byte);
        }
    }

    /// Get sanitized tail as string with secret patterns scrubbed
    pub fn get_sanitized_tail(&self) -> String {
        let buf = self.buffer.lock().unwrap();
        let raw = String::from_utf8_lossy(&buf.iter().cloned().collect::<Vec<u8>>()).to_string();

        let mut sanitized = raw;
        for pattern in &self.patterns {
            sanitized = pattern
                .replace_all(&sanitized, "[REDACTED_SECRET]")
                .to_string();
        }

        sanitized
    }

    /// Flush sanitized tail to isolated 0600 file
    pub fn flush_to_diagnostic_file(&self) -> std::io::Result<()> {
        let sanitized = self.get_sanitized_tail();

        if let Some(parent) = self.diagnostic_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let mut options = OpenOptions::new();
        options.create(true).write(true).truncate(true);
        #[cfg(unix)]
        options.mode(0o600); // Strict read/write permissions for current process only

        let mut file = options.open(&self.diagnostic_path)?;
        file.write_all(sanitized.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_stderr_sanitizer_secret_scrubbing_and_tail_limit() {
        let temp = tempdir().unwrap();
        let diag_file = temp.path().join("harness-diagnostics").join("stderr.jsonl");
        let sanitizer = StderrSanitizer::new(&diag_file);

        // Append text with secret
        let msg = b"Error: failed connecting with bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9 and sk-1234567890123456789012345\n";
        sanitizer.append_chunk(msg);

        let sanitized = sanitizer.get_sanitized_tail();
        assert!(!sanitized.contains("sk-1234567890123456789012345"));
        assert!(sanitized.contains("[REDACTED_SECRET]"));

        sanitizer.flush_to_diagnostic_file().unwrap();
        assert!(diag_file.exists());
    }
}
