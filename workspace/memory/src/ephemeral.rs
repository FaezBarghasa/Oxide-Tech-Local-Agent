use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};

/// An entry in the active terminal session ephemeral ring buffer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TerminalBufferEntry {
    pub session_id: String,
    pub command: String,
    pub output_snippet: String,
    pub exit_code: Option<i32>,
    pub timestamp: DateTime<Utc>,
}

/// An open editor buffer/tab in the ephemeral memory
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenEditorBuffer {
    pub file_path: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub selection: Option<String>,
    pub is_dirty: bool,
    pub last_accessed: DateTime<Utc>,
}

/// A captured runtime stack trace or panic diagnostic
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActiveStackTrace {
    pub trace_id: String,
    pub error_type: String,
    pub message: String,
    pub frames: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

/// Thread-safe Ephemeral Memory Layer for active session states
#[derive(Debug, Clone)]
pub struct EphemeralMemory {
    max_terminal_entries: usize,
    terminal_buffer: Arc<RwLock<VecDeque<TerminalBufferEntry>>>,
    open_buffers: Arc<RwLock<Vec<OpenEditorBuffer>>>,
    active_traces: Arc<RwLock<VecDeque<ActiveStackTrace>>>,
}

impl EphemeralMemory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_terminal_entries: max_entries,
            terminal_buffer: Arc::new(RwLock::new(VecDeque::with_capacity(max_entries))),
            open_buffers: Arc::new(RwLock::new(Vec::new())),
            active_traces: Arc::new(RwLock::new(VecDeque::with_capacity(20))),
        }
    }

    /// Record a command execution into the ring buffer
    pub fn record_terminal_output(
        &self,
        session_id: &str,
        command: &str,
        output_snippet: &str,
        exit_code: Option<i32>,
    ) {
        let mut buf = self.terminal_buffer.write().unwrap();
        if buf.len() >= self.max_terminal_entries {
            buf.pop_front();
        }
        buf.push_back(TerminalBufferEntry {
            session_id: session_id.to_string(),
            command: command.to_string(),
            output_snippet: output_snippet.to_string(),
            exit_code,
            timestamp: Utc::now(),
        });
    }

    /// Update or register an open editor buffer state
    pub fn update_editor_buffer(
        &self,
        file_path: &str,
        cursor_line: usize,
        cursor_col: usize,
        selection: Option<String>,
        is_dirty: bool,
    ) {
        let mut bufs = self.open_buffers.write().unwrap();
        if let Some(existing) = bufs.iter_mut().find(|b| b.file_path == file_path) {
            existing.cursor_line = cursor_line;
            existing.cursor_col = cursor_col;
            existing.selection = selection;
            existing.is_dirty = is_dirty;
            existing.last_accessed = Utc::now();
        } else {
            bufs.push(OpenEditorBuffer {
                file_path: file_path.to_string(),
                cursor_line,
                cursor_col,
                selection,
                is_dirty,
                last_accessed: Utc::now(),
            });
        }
    }

    /// Capture an active panic or runtime error stack trace
    pub fn push_stack_trace(&self, error_type: &str, message: &str, frames: Vec<String>) {
        let mut traces = self.active_traces.write().unwrap();
        if traces.len() >= 20 {
            traces.pop_front();
        }
        traces.push_back(ActiveStackTrace {
            trace_id: format!("trace_{}", Utc::now().timestamp_millis()),
            error_type: error_type.to_string(),
            message: message.to_string(),
            frames,
            timestamp: Utc::now(),
        });
    }

    /// Retrieve the most recent terminal logs
    pub fn get_recent_terminal_logs(&self, limit: usize) -> Vec<TerminalBufferEntry> {
        let buf = self.terminal_buffer.read().unwrap();
        buf.iter().rev().take(limit).cloned().collect()
    }

    /// Retrieve active open editor files
    pub fn get_open_buffers(&self) -> Vec<OpenEditorBuffer> {
        let bufs = self.open_buffers.read().unwrap();
        bufs.clone()
    }

    /// Retrieve the latest stack trace if present
    pub fn get_latest_stack_trace(&self) -> Option<ActiveStackTrace> {
        let traces = self.active_traces.read().unwrap();
        traces.back().cloned()
    }
}

impl Default for EphemeralMemory {
    fn default() -> Self {
        Self::new(50)
    }
}
