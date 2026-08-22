use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OscillationDetector {
    max_repeats: usize,
    history: Vec<String>,
    call_counts: HashMap<String, usize>,
}

impl OscillationDetector {
    pub fn new(max_repeats: usize) -> Self {
        Self {
            max_repeats,
            history: Vec::new(),
            call_counts: HashMap::new(),
        }
    }

    /// Record a tool call signature (e.g., "tool_name:args_hash")
    /// Returns Ok(()) if safe, or Err(alert_message) if oscillation threshold is exceeded.
    pub fn record_and_check(&mut self, call_sig: &str) -> Result<(), String> {
        let count = self.call_counts.entry(call_sig.to_string()).or_insert(0);
        *count += 1;
        self.history.push(call_sig.to_string());

        if *count >= self.max_repeats {
            let msg = format!(
                "Oscillation detected! Tool call '{}' was executed {} times (threshold = {}). Breaking loop.",
                call_sig, *count, self.max_repeats
            );
            tracing::warn!("{}", msg);
            return Err(msg);
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.history.clear();
        self.call_counts.clear();
    }
}

impl Default for OscillationDetector {
    fn default() -> Self {
        Self::new(3)
    }
}
