---
okf_version: "0.2"
type: Function
title: complete
description: "Send a system + user prompt and return the model's text completion."
resource: crates/vllm-client/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/vllm-client/src/client/complete
language: rust
---

# complete

Send a system + user prompt and return the model's text completion.

## Signature

```rust
impl LlmRouterClient { pub fn complete(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        expect_json: bool,
    ) -> Result<String, anyhow::Error> }
```

## Visibility

- `pub`

## Docstring

Send a system + user prompt and return the model's text completion.

`expect_json` instructs OpenAI-compatible backends to use
`response_format: { type: "json_object" }`.  Ollama ignores this flag
(JSON mode is controlled via the system prompt instead).
[tracing::instrument(name = "llm_complete", skip(self, system_prompt, user_prompt), fields(provider = ?self.provider, model = %self.model, expect_json = expect_json))]

## Source
Lines 200–234 in `crates/vllm-client/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/vllm-client/src/client.md) |
