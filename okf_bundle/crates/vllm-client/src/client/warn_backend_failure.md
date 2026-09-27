---
okf_version: "0.2"
type: Function
title: warn_backend_failure
description: "Emit a diagnostic warning when the current backend fails, visible in"
resource: crates/vllm-client/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/vllm-client/src/client/warn_backend_failure
language: rust
---

# warn_backend_failure

Emit a diagnostic warning when the current backend fails, visible in

## Signature

```rust
impl LlmRouterClient { pub fn warn_backend_failure(&self, reason: &str) }
```

## Visibility

- `pub`

## Docstring

Emit a diagnostic warning when the current backend fails, visible in
structured tracing logs.

## Source
Lines 379–386 in `crates/vllm-client/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/vllm-client/src/client.md) |
