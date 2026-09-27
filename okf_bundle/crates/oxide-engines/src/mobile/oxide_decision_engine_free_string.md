---
okf_version: "0.2"
type: Function
title: oxide_decision_engine_free_string
description: "Free a C-string allocated by `oxide_decision_engine_decide_json`."
resource: crates/oxide-engines/src/mobile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:53:11Z"
concept_id: crates/oxide-engines/src/mobile/oxide_decision_engine_free_string
language: rust
---

# oxide_decision_engine_free_string

Free a C-string allocated by `oxide_decision_engine_decide_json`.

## Signature

```rust
pub fn oxide_decision_engine_free_string(str_ptr: *mut c_char)
```

## Decorators

- `unsafe(no_mangle)`

## Visibility

- `pub`

## Docstring

Free a C-string allocated by `oxide_decision_engine_decide_json`.

# Safety
`str_ptr` must be a valid pointer allocated by Rust or NULL.
[unsafe(no_mangle)]

## Source
Lines 140–146 in `crates/oxide-engines/src/mobile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile](/crates/oxide-engines/src/mobile.md) |
| called_by | [test_c_ffi_json_bridge](/crates/oxide-engines/src/mobile/test_c_ffi_json_bridge.md) |
