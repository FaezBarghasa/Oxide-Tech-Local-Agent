---
okf_version: "0.2"
type: Function
title: tool_result
resource: crates/vllm-client/src/provider.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/provider/tool_result_1
language: rust
---

# tool_result

## Signature

```rust
pub fn tool_result(
        tool_call_id: impl Into<String>,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 118–130 in `crates/vllm-client/src/provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [provider](/crates/vllm-client/src/provider.md) |
