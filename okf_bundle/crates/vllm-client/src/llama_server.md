---
okf_version: "0.2"
type: Module
title: llama_server
description: llama-server (llama.cpp HTTP) provider implementation.
resource: crates/vllm-client/src/llama_server.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/llama_server
language: rust
---

# llama_server

llama-server (llama.cpp HTTP) provider implementation.

## Docstring

llama-server (llama.cpp HTTP) provider implementation.

Supports:
- OpenAI-compatible `/v1/chat/completions` (streaming SSE + blocking)
- Native grammar-constrained output via the `grammar` GBNF field
- Tool-call parsing from `tool_calls` response field
- KV prefix-cache slot stickiness via `id_slot`
- `/health` endpoint monitoring with VRAM and slot stats

## Relationships

| Type | Target |
|------|--------|
| related | [LlamaServerProvider](/crates/vllm-client/src/llama_server/LlamaServerProvider.md) |
| related | [new](/crates/vllm-client/src/llama_server/new.md) |
| related | [build_messages](/crates/vllm-client/src/llama_server/build_messages.md) |
| related | [new](/crates/vllm-client/src/llama_server/new.md) |
| related | [build_messages](/crates/vllm-client/src/llama_server/build_messages.md) |
| related | [OaiToolCallFunction](/crates/vllm-client/src/llama_server/OaiToolCallFunction.md) |
| related | [OaiToolCall](/crates/vllm-client/src/llama_server/OaiToolCall.md) |
| related | [OaiMessage](/crates/vllm-client/src/llama_server/OaiMessage.md) |
| related | [OaiChoice](/crates/vllm-client/src/llama_server/OaiChoice.md) |
| related | [OaiUsage](/crates/vllm-client/src/llama_server/OaiUsage.md) |
| related | [OaiChatResponse](/crates/vllm-client/src/llama_server/OaiChatResponse.md) |
| related | [LlamaHealth](/crates/vllm-client/src/llama_server/LlamaHealth.md) |
| related | [capabilities](/crates/vllm-client/src/llama_server/capabilities.md) |
| related | [provider_name](/crates/vllm-client/src/llama_server/provider_name.md) |
| related | [chat_completion](/crates/vllm-client/src/llama_server/chat_completion.md) |
| related | [stream_chat](/crates/vllm-client/src/llama_server/stream_chat.md) |
| related | [health](/crates/vllm-client/src/llama_server/health.md) |
| related | [capabilities](/crates/vllm-client/src/llama_server/capabilities.md) |
| related | [provider_name](/crates/vllm-client/src/llama_server/provider_name.md) |
| related | [chat_completion](/crates/vllm-client/src/llama_server/chat_completion.md) |
| related | [stream_chat](/crates/vllm-client/src/llama_server/stream_chat.md) |
| related | [health](/crates/vllm-client/src/llama_server/health.md) |
| related | [anyhow](/_dependencies/cargo/anyhow.md) |
| related | [reqwest](/_dependencies/cargo/reqwest.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
