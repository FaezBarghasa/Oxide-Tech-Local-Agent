---
okf_version: "0.2"
type: Function
title: route_for_domain
resource: crates/vllm-client/src/lora_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/vllm-client/src/lora_router/route_for_domain
language: rust
---

# route_for_domain

## Signature

```rust
impl DynamicLoraRouter { pub fn route_for_domain(&self, domain: &str) -> Result<String> }
```

## Visibility

- `pub`

## Source
Lines 17–44 in `crates/vllm-client/src/lora_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lora_router](/crates/vllm-client/src/lora_router.md) |
