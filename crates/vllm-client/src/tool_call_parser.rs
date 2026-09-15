use crate::provider::ToolCall;
use serde_json::Value;
use uuid::Uuid;

pub struct ToolCallParser;

impl ToolCallParser {
    /// Attempt to parse tool calls from various model outputs (JSON, markdown codeblocks, XML tags).
    pub fn extract_from_text(content: &str) -> Vec<ToolCall> {
        let mut results = Vec::new();

        // 1. Check for XML-style `<tool_call>` or `<tool_use>`
        if let Some(tool_calls) = Self::parse_xml_tags(content) {
            if !tool_calls.is_empty() {
                return tool_calls;
            }
        }

        // 2. Check for markdown json block containing tool invocation
        if let Some(tool_calls) = Self::parse_markdown_json(content) {
            if !tool_calls.is_empty() {
                return tool_calls;
            }
        }

        // 3. Check for direct JSON objects { "name": ..., "arguments": ... }
        if let Some(tool_call) = Self::parse_direct_json(content) {
            results.push(tool_call);
        }

        results
    }

    fn parse_xml_tags(content: &str) -> Option<Vec<ToolCall>> {
        let mut calls = Vec::new();
        let mut cursor = content;

        while let Some(start) = cursor.find("<tool_call>") {
            let rest = &cursor[start + "<tool_call>".len()..];
            if let Some(end) = rest.find("</tool_call>") {
                let inner = &rest[..end].trim();
                if let Ok(val) = serde_json::from_str::<Value>(inner) {
                    if let Some(call) = Self::value_to_tool_call(&val) {
                        calls.push(call);
                    }
                }
                cursor = &rest[end + "</tool_call>".len()..];
            } else {
                break;
            }
        }

        if calls.is_empty() {
            None
        } else {
            Some(calls)
        }
    }

    fn parse_markdown_json(content: &str) -> Option<Vec<ToolCall>> {
        let mut calls = Vec::new();
        let mut cursor = content;

        while let Some(start) = cursor.find("```json") {
            let rest = &cursor[start + "```json".len()..];
            if let Some(end) = rest.find("```") {
                let inner = rest[..end].trim();
                if let Ok(val) = serde_json::from_str::<Value>(inner) {
                    if let Some(call) = Self::value_to_tool_call(&val) {
                        calls.push(call);
                    } else if let Some(arr) = val.as_array() {
                        for item in arr {
                            if let Some(c) = Self::value_to_tool_call(item) {
                                calls.push(c);
                            }
                        }
                    }
                }
                cursor = &rest[end + "```".len()..];
            } else {
                break;
            }
        }

        if calls.is_empty() {
            None
        } else {
            Some(calls)
        }
    }

    fn parse_direct_json(content: &str) -> Option<ToolCall> {
        let trimmed = content.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            if let Ok(val) = serde_json::from_str::<Value>(trimmed) {
                return Self::value_to_tool_call(&val);
            }
        }
        None
    }

    fn value_to_tool_call(val: &Value) -> Option<ToolCall> {
        if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
            let arguments = val.get("arguments").cloned().unwrap_or(Value::Null);
            let id = val
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("call_{}", Uuid::new_v4().simple()));
            return Some(ToolCall {
                id,
                name: name.to_string(),
                arguments,
            });
        }
        if let Some(action) = val.get("action").and_then(|v| v.as_str()) {
            let arguments = val.get("parameters").cloned().unwrap_or(Value::Null);
            let id = format!("call_{}", Uuid::new_v4().simple());
            return Some(ToolCall {
                id,
                name: action.to_string(),
                arguments,
            });
        }
        None
    }
}
