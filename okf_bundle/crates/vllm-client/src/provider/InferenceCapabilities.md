---
okf_version: "0.2"
type: Class
title: InferenceCapabilities
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/vllm-client/src/provider.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/provider/InferenceCapabilities
language: rust
---

# InferenceCapabilities

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct InferenceCapabilities
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `provider`
- `supports_streaming`
- `supports_tool_calls`
- `supports_lora_hotswap`
- `supports_json_mode`
- `supports_grammar_constrained`
- `supports_speculative_decoding`
- `supports_prefix_cache`
- `supports_multimodal`
- `context_window`

## Source
Lines 23–34 in `crates/vllm-client/src/provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [provider](/crates/vllm-client/src/provider.md) |
