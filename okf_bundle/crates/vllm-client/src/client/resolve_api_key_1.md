---
okf_version: "0.2"
type: Function
title: resolve_api_key
description: Resolve the bearer token for the current provider from env vars.
resource: crates/vllm-client/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/vllm-client/src/client/resolve_api_key_1
language: rust
---

# resolve_api_key

Resolve the bearer token for the current provider from env vars.

## Signature

```rust
fn resolve_api_key(&self) -> Option<String>
```

## Docstring

Resolve the bearer token for the current provider from env vars.

## Source
Lines 300–332 in `crates/vllm-client/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/vllm-client/src/client.md) |
