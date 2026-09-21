use oxide_core::ChatMessage;

pub struct DatasetFormatter;

impl DatasetFormatter {
    pub fn format_chatml(messages: &[ChatMessage]) -> String {
        let mut out = String::new();
        for msg in messages {
            let role = match msg.role {
                oxide_core::Role::System => "system",
                oxide_core::Role::User => "user",
                oxide_core::Role::Assistant => "assistant",
                oxide_core::Role::Tool => "tool",
            };
            out.push_str(&format!("<|im_start|>{}\n{}<|im_end|>\n", role, msg.content));
        }
        out.push_str("<|im_start|>assistant\n");
        out
    }
}

pub struct ContextCompactor {
    pub max_tokens: usize,
}

impl ContextCompactor {
    pub fn new(max_tokens: usize) -> Self {
        Self { max_tokens }
    }

    pub fn should_compact(&self, estimated_tokens: usize) -> bool {
        estimated_tokens > (self.max_tokens * 8) / 10
    }

    pub fn compact_history(&self, messages: &mut Vec<ChatMessage>, summary: &str) {
        if messages.len() <= 2 {
            return;
        }
        // Keep system prompt if present, inject summary, preserve latest turns
        let mut compacted = Vec::new();
        if let Some(first) = messages.first() {
            if matches!(first.role, oxide_core::Role::System) {
                compacted.push(first.clone());
            }
        }
        compacted.push(ChatMessage {
            role: oxide_core::Role::System,
            content: format!("[SYSTEM MEMORY SUMMARY]: {}", summary).into(),
            name: None,
        });

        // Retain last 2 messages
        let tail_start = messages.len().saturating_sub(2);
        compacted.extend(messages[tail_start..].iter().cloned());
        *messages = compacted;
    }
}

pub mod ornith_formatter;
pub use ornith_formatter::{ExtractedToolCall, OrnithPromptFormatter};


