use oxide_core::ChatMessage;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtractedToolCall {
    pub name: String,
    pub arguments: Value,
}

/// Prompt formatter and tool-call extractor tailored for the Ornith-1.5 agentic coding model.
pub struct OrnithPromptFormatter;

impl OrnithPromptFormatter {
    /// Format conversation history and active tool definitions into Ornith-1.5's ChatML template.
    pub fn format(
        messages: &[ChatMessage],
        tools: Option<&[Value]>,
        custom_system_prompt: Option<&str>,
    ) -> String {
        let mut out = String::new();

        // 1. Build System Block
        out.push_str("<|im_start|>system\n");
        if let Some(sys) = custom_system_prompt {
            out.push_str(sys);
            out.push('\n');
        } else {
            out.push_str("You are Ornith-1.5, a high-performance autonomous agent specialized in systems engineering, low-level Rust development, and precise tool execution.\n");
        }

        if let Some(tool_list) = tools {
            if !tool_list.is_empty() {
                out.push_str("\n# Tools\n");
                out.push_str("You may call one or more functions to assist with the user query.\n");
                out.push_str("You are provided with function signatures within <tools></tools> XML tags:\n<tools>\n");
                for tool in tool_list {
                    out.push_str(&serde_json::to_string(tool).unwrap_or_default());
                    out.push('\n');
                }
                out.push_str("</tools>\n\n");
                out.push_str("For each function call, return a json object with function name and arguments within <tool_call></tool_call> XML tags:\n");
                out.push_str("<tool_call>\n{\"name\": \"<function-name>\", \"arguments\": <args-json-object>}\n</tool_call>\n");
            }
        }
        out.push_str("<|im_end|>\n");

        // 2. Append Chat Messages
        for msg in messages {
            let role = match msg.role {
                oxide_core::Role::System => continue, // Handled above
                oxide_core::Role::User => "user",
                oxide_core::Role::Assistant => "assistant",
                oxide_core::Role::Tool => "tool",
            };
            out.push_str(&format!("<|im_start|>{}\n{}<|im_end|>\n", role, msg.content));
        }

        // 3. Priming prompt for generation
        out.push_str("<|im_start|>assistant\n");
        out
    }

    /// Extract tool calls from an Ornith-1.5 raw model completion output.
    pub fn extract_tool_calls(completion: &str) -> Vec<ExtractedToolCall> {
        let mut calls = Vec::new();
        let mut cursor = completion;

        while let Some(start_idx) = cursor.find("<tool_call>") {
            let rest = &cursor[start_idx + "<tool_call>".len()..];
            if let Some(end_idx) = rest.find("</tool_call>") {
                let json_str = rest[..end_idx].trim();
                if let Ok(val) = serde_json::from_str::<Value>(json_str) {
                    if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                        let args = val.get("arguments").cloned().unwrap_or(Value::Null);
                        calls.push(ExtractedToolCall {
                            name: name.to_string(),
                            arguments: args,
                        });
                    }
                }
                cursor = &rest[end_idx + "</tool_call>".len()..];
            } else {
                break;
            }
        }

        calls
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_core::Role;
    use serde_json::json;

    #[test]
    fn test_ornith_formatter_with_tools() {
        let tools = vec![json!({
            "type": "function",
            "function": {
                "name": "cargo_check",
                "description": "Run cargo check on target workspace crate"
            }
        })];

        let msgs = vec![ChatMessage::new_text(Role::User, "Check the core crate")];

        let formatted = OrnithPromptFormatter::format(&msgs, Some(&tools), None);
        assert!(formatted.contains("<tools>"));
        assert!(formatted.contains("cargo_check"));
        assert!(formatted.contains("<|im_start|>user\nCheck the core crate<|im_end|>"));
        assert!(formatted.ends_with("<|im_start|>assistant\n"));
    }

    #[test]
    fn test_extract_tool_calls() {
        let completion = r#"
I will check the crate now.
<tool_call>
{"name": "cargo_check", "arguments": {"crate_name": "oxide-core"}}
</tool_call>
Done!
"#;

        let tool_calls = OrnithPromptFormatter::extract_tool_calls(completion);
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].name, "cargo_check");
        assert_eq!(
            tool_calls[0].arguments["crate_name"],
            json!("oxide-core")
        );
    }
}
