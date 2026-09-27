---
okf_version: "0.2"
type: Function
title: emit
resource: crates/forge-rust/src/emitter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/emitter/emit_1
language: rust
---

# emit

## Signature

```rust
pub fn emit(module: &UirModule) -> String
```

## Visibility

- `pub`

## Source
Lines 10–64 in `crates/forge-rust/src/emitter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emitter](/crates/forge-rust/src/emitter.md) |
| calls | [emit_struct](/crates/forge-rust/src/emitter/emit_struct.md) |
| calls | [emit_enum](/crates/forge-rust/src/emitter/emit_enum.md) |
| calls | [emit_function](/crates/forge-rust/src/emitter/emit_function.md) |
| calls | [emit_trait](/crates/forge-rust/src/emitter/emit_trait.md) |
| calls | [emit_const](/crates/forge-rust/src/emitter/emit_const.md) |
