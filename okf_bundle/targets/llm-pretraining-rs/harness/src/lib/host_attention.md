---
okf_version: "0.2"
type: Function
title: host_attention
resource: targets/llm-pretraining-rs/harness/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/harness/src/lib/host_attention
language: rust
---

# host_attention

## Signature

```rust
fn host_attention(
    input: TensorHandle,
    _n_head: u32,
    _qknorm: bool,
    _rope_theta: f64,
    _flash: bool,
) -> TensorHandle
```

## Source
Lines 35–43 in `targets/llm-pretraining-rs/harness/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/harness/src/lib.md) |
