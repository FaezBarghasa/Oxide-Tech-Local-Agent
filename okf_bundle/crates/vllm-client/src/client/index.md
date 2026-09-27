# client

## Classs

- [ChatMessage](ChatMessage.md) — [derive(Debug, Serialize, Clone)]
- [LlmProvider](LlmProvider.md) — The backing inference provider for a `LlmRouterClient` instance.
- [LlmRouterClient](LlmRouterClient.md) — A unified LLM client that dispatches to Groq, Mistral, Ollama, or a
- [OllamaMessage](OllamaMessage.md) — [derive(Debug, Serialize, Deserialize, Clone)]
- [OllamaOptions](OllamaOptions.md) — [derive(Debug, Serialize, Clone)]
- [OllamaRequest](OllamaRequest.md) — [derive(Debug, Serialize, Clone)]
- [OllamaResponse](OllamaResponse.md) — [derive(Debug, Deserialize, Clone)]
- [OpenAiChoice](OpenAiChoice.md) — [derive(Debug, Deserialize, Clone)]
- [OpenAiChoiceMessage](OpenAiChoiceMessage.md) — [derive(Debug, Deserialize, Clone)]
- [OpenAiRequest](OpenAiRequest.md) — [derive(Debug, Serialize, Clone)]
- [OpenAiResponse](OpenAiResponse.md) — [derive(Debug, Deserialize, Clone)]
- [ResponseFormat](ResponseFormat.md) — [derive(Debug, Serialize, Clone)]

## Functions

- [complete](complete.md) — Send a system + user prompt and return the model's text completion.
- [complete](complete_1.md) — Send a system + user prompt and return the model's text completion.
- [complete_ollama](complete_ollama.md) — [tracing::instrument(name = "llm_ollama_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]
- [complete_ollama](complete_ollama_1.md) — [tracing::instrument(name = "llm_ollama_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]
- [complete_openai_compatible](complete_openai_compatible.md) — [tracing::instrument(name = "llm_openai_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]
- [complete_openai_compatible](complete_openai_compatible_1.md) — [tracing::instrument(name = "llm_openai_request", skip(self, system_prompt, user_prompt), fields(url = %self.base_url))]
- [from_config](from_config.md) — Construct from a `ModelConfig` slice (parsed from `config.toml`).
- [from_config](from_config_1.md) — Construct from a `ModelConfig` slice (parsed from `config.toml`).
- [from_str](from_str.md)
- [from_str](from_str_1.md)
- [ollama](ollama.md) — Build an explicit Ollama client (convenience constructor).
- [ollama](ollama_1.md) — Build an explicit Ollama client (convenience constructor).
- [resolve_api_key](resolve_api_key.md) — Resolve the bearer token for the current provider from env vars.
- [resolve_api_key](resolve_api_key_1.md) — Resolve the bearer token for the current provider from env vars.
- [warn_backend_failure](warn_backend_failure.md) — Emit a diagnostic warning when the current backend fails, visible in
- [warn_backend_failure](warn_backend_failure_1.md) — Emit a diagnostic warning when the current backend fails, visible in
