---
okf_version: "0.2"
type: Function
title: register_model
resource: crates/oxide-state/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:00:09Z"
concept_id: crates/oxide-state/src/lib/register_model
language: rust
---

# register_model

## Signature

```rust
impl AppState { pub fn register_model(&self, name: impl Into<String>, provider: Arc<dyn InferenceProvider>) }
```

## Visibility

- `pub`

## Source
Lines 81–83 in `crates/oxide-state/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-state/src/lib.md) |
