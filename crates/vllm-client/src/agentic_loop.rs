use crate::provider::{ChatMessage, ChatRequest, ConversationTurn, InferenceProvider, ToolCall, ToolDefinition};
use crate::tool_call_parser::ToolCallParser;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

/// Trait for executing tool calls (e.g., local MCP servers or system tools).
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute_tool(&self, name: &str, arguments: &Value) -> Result<String>;
}

/// Dynamic closure or mock-based tool executor.
pub struct SimpleToolExecutor<F>
where
    F: Fn(&str, &Value) -> Result<String> + Send + Sync,
{
    handler: F,
}

impl<F> SimpleToolExecutor<F>
where
    F: Fn(&str, &Value) -> Result<String> + Send + Sync,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait]
impl<F> ToolExecutor for SimpleToolExecutor<F>
where
    F: Fn(&str, &Value) -> Result<String> + Send + Sync,
{
    async fn execute_tool(&self, name: &str, arguments: &Value) -> Result<String> {
        (self.handler)(name, arguments)
    }
}

/// Agentic runner managing multi-turn conversation state, tool calls, and oscillation guardrails.
pub struct AgenticLoopRunner {
    provider: Arc<dyn InferenceProvider>,
    tool_executor: Arc<dyn ToolExecutor>,
    tool_definitions: Vec<ToolDefinition>,
    max_turns: usize,
    oscillation_threshold: usize,
}

impl AgenticLoopRunner {
    pub fn new(
        provider: Arc<dyn InferenceProvider>,
        tool_executor: Arc<dyn ToolExecutor>,
        tool_definitions: Vec<ToolDefinition>,
        max_turns: usize,
    ) -> Self {
        Self {
            provider,
            tool_executor,
            tool_definitions,
            max_turns: max_turns.max(1),
            oscillation_threshold: 3,
        }
    }

    /// Execute the autonomous agent loop until completion or max turns reached.
    pub async fn run(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<(String, Vec<ConversationTurn>)> {
        let mut history: Vec<ConversationTurn> = Vec::new();
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ];

        let mut turn_counter = 0;
        let mut executed_signatures: Vec<String> = Vec::new();

        while turn_counter < self.max_turns {
            turn_counter += 1;

            let req = ChatRequest {
                model: String::new(),
                messages: messages.clone(),
                temperature: Some(0.1),
                max_tokens: Some(4096),
                json_mode: None,
                history: history.clone(),
                tools: Some(self.tool_definitions.clone()),
                tool_choice: Some("auto".to_string()),
                images: None,
                grammar: None,
                stop: None,
                slot_id: None,
            };

            let response = self.provider.chat_completion(req).await?;
            let mut detected_tool_calls = response.tool_calls.clone();

            // If the provider didn't parse structured tool_calls, use fallback parser
            if detected_tool_calls.is_empty() {
                detected_tool_calls = ToolCallParser::extract_from_text(&response.content);
            }

            if detected_tool_calls.is_empty() {
                // No tool call requested — agent provided its final answer
                history.push(ConversationTurn {
                    role: "assistant".to_string(),
                    content: response.content.clone(),
                    tool_calls: vec![],
                    tool_call_id: None,
                    name: None,
                });
                return Ok((response.content, history));
            }

            // Record assistant turn
            history.push(ConversationTurn {
                role: "assistant".to_string(),
                content: response.content.clone(),
                tool_calls: detected_tool_calls.clone(),
                tool_call_id: None,
                name: None,
            });

            messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: response.content.clone(),
            });

            // Execute each tool call and check oscillation
            for tc in &detected_tool_calls {
                let signature = format!("{}:{}", tc.name, tc.arguments);
                let repeated_count = executed_signatures.iter().filter(|&s| s == &signature).count();

                if repeated_count >= self.oscillation_threshold {
                    let err_msg = format!(
                        "Oscillation guardrail tripped: Tool '{}' called {} times with identical parameters.",
                        tc.name, repeated_count
                    );
                    tracing::warn!("{}", err_msg);

                    messages.push(ChatMessage {
                        role: "tool".to_string(),
                        content: format!("ERROR: You are looping on tool '{}' with identical arguments. Please revise your strategy or return the final answer.", tc.name),
                    });
                    continue;
                }

                executed_signatures.push(signature);

                let tool_output = match self.tool_executor.execute_tool(&tc.name, &tc.arguments).await {
                    Ok(out) => out,
                    Err(e) => format!("Tool execution error: {}", e),
                };

                let tool_call_id = if tc.id.is_empty() {
                    format!("call_{}", turn_counter)
                } else {
                    tc.id.clone()
                };

                history.push(ConversationTurn {
                    role: "tool".to_string(),
                    content: tool_output.clone(),
                    tool_calls: vec![],
                    tool_call_id: Some(tool_call_id),
                    name: Some(tc.name.clone()),
                });

                messages.push(ChatMessage {
                    role: "tool".to_string(),
                    content: tool_output,
                });
            }
        }

        Err(anyhow!(
            "Agent reached maximum turn limit ({}) without terminating.",
            self.max_turns
        ))
    }
}
