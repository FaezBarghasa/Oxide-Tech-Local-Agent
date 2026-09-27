# provider

## Classs

- [BackendHealth](BackendHealth.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ChatMessage](ChatMessage.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ChatRequest](ChatRequest.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ChatResponse](ChatResponse.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ConversationTurn](ConversationTurn.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [InferenceCapabilities](InferenceCapabilities.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [InferenceProvider](InferenceProvider.md) — Pluggable inference provider abstraction across Ollama, SGLang,
- [ProviderKind](ProviderKind.md) — [derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
- [StreamChunk](StreamChunk.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ToolCall](ToolCall.md) — [derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
- [ToolDefinition](ToolDefinition.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ToolParameter](ToolParameter.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ToolParameters](ToolParameters.md) — [derive(Debug, Clone, Serialize, Deserialize)]

## Functions

- [activate_lora](activate_lora.md) — Activate a LoRA adapter by name/path. No-op if not supported.
- [assistant](assistant.md)
- [assistant](assistant_1.md)
- [deactivate_lora](deactivate_lora.md) — Deactivate / unload the active LoRA adapter.
- [provider_name](provider_name.md) — Provider display name for logging and telemetry.
- [system](system.md)
- [system](system_1.md)
- [tool_result](tool_result.md)
- [tool_result](tool_result_1.md)
- [user](user.md)
- [user](user_1.md)
