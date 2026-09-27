---
okf_version: "0.2"
type: Function
title: oxide_decision_engine_free
description: "Free a `DecisionEngine` instance allocated by `oxide_decision_engine_create`."
resource: crates/oxide-engines/src/mobile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:53:11Z"
concept_id: crates/oxide-engines/src/mobile/oxide_decision_engine_free
language: rust
---

# oxide_decision_engine_free

Free a `DecisionEngine` instance allocated by `oxide_decision_engine_create`.

## Signature

```rust
pub fn oxide_decision_engine_free(engine_ptr: *mut DecisionEngine)
```

## Decorators

- `unsafe(no_mangle)`

## Visibility

- `pub`

## Docstring

Free a `DecisionEngine` instance allocated by `oxide_decision_engine_create`.

# Safety
`engine_ptr` must be a valid pointer created by `oxide_decision_engine_create` or NULL.
[unsafe(no_mangle)]

## Source
Lines 83–89 in `crates/oxide-engines/src/mobile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile](/crates/oxide-engines/src/mobile.md) |
| called_by | [test_c_ffi_json_bridge](/crates/oxide-engines/src/mobile/test_c_ffi_json_bridge.md) |
