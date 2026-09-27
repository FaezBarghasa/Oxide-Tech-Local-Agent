---
okf_version: "0.2"
type: Function
title: host_linear
resource: targets/llm-pretraining-rs/harness/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/harness/src/lib/host_linear
language: rust
---

# host_linear

## Signature

```rust
fn host_linear(
    _input: TensorHandle,
    _out_dim: u32,
    _in_dim: u32,
    _name: *const u8,
    _name_len: usize,
    _model: ModelHandle,
) -> TensorHandle
```

## Source
Lines 20–29 in `targets/llm-pretraining-rs/harness/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/targets/llm-pretraining-rs/harness/src/lib.md) |
| calls | [TensorHandle](/crates/surface-api/src/abi/TensorHandle.md) |
