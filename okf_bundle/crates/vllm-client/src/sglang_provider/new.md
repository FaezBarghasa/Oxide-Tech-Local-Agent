---
okf_version: "0.2"
type: Function
title: new
resource: crates/vllm-client/src/sglang_provider.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:58:19Z"
concept_id: crates/vllm-client/src/sglang_provider/new
language: rust
---

# new

## Signature

```rust
impl SglangProvider { pub fn new(
        base_url: Option<String>,
        default_model: Option<String>,
        draft_model: Option<String>,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 21–32 in `crates/vllm-client/src/sglang_provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sglang_provider](/crates/vllm-client/src/sglang_provider.md) |
