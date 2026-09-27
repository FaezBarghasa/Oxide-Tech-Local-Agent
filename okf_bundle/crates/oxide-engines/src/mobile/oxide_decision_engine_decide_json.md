---
okf_version: "0.2"
type: Function
title: oxide_decision_engine_decide_json
description: "Execute a synchronous non-autoregressive decision pass from a JSON-serialized `DecisionInput`."
resource: crates/oxide-engines/src/mobile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:53:11Z"
concept_id: crates/oxide-engines/src/mobile/oxide_decision_engine_decide_json
language: rust
---

# oxide_decision_engine_decide_json

Execute a synchronous non-autoregressive decision pass from a JSON-serialized `DecisionInput`.

## Signature

```rust
pub fn oxide_decision_engine_decide_json(
    engine_ptr: *const DecisionEngine,
    input_json_ptr: *const c_char,
) -> *mut c_char
```

## Decorators

- `unsafe(no_mangle)`

## Visibility

- `pub`

## Docstring

Execute a synchronous non-autoregressive decision pass from a JSON-serialized `DecisionInput`.

Returns a JSON-serialized `DecisionOutput` string.

# Safety
`engine_ptr` and `input_json_ptr` must be valid, non-null pointers.
Returned string must be freed with `oxide_decision_engine_free_string`.
[unsafe(no_mangle)]

## Source
Lines 99–133 in `crates/oxide-engines/src/mobile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile](/crates/oxide-engines/src/mobile.md) |
| called_by | [test_c_ffi_json_bridge](/crates/oxide-engines/src/mobile/test_c_ffi_json_bridge.md) |
