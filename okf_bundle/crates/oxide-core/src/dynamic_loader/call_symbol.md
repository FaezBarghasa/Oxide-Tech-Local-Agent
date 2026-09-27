---
okf_version: "0.2"
type: Function
title: call_symbol
description: Execute a symbol from a loaded library behind a panic-isolated FFI boundary.
resource: crates/oxide-core/src/dynamic_loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/dynamic_loader/call_symbol
language: rust
---

# call_symbol

Execute a symbol from a loaded library behind a panic-isolated FFI boundary.

## Signature

```rust
impl DynamicSkillLoader { pub fn call_symbol(
        &self,
        lib_name: &str,
        symbol_name: &str,
        input_data: &[u8],
        output_buffer: &mut [u8],
    ) -> Result<i32, OxideError> }
```

## Visibility

- `pub`

## Docstring

Execute a symbol from a loaded library behind a panic-isolated FFI boundary.

## Source
Lines 54–88 in `crates/oxide-core/src/dynamic_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dynamic_loader](/crates/oxide-core/src/dynamic_loader.md) |
| calls | [call_ffi_safe](/crates/oxide-core/src/ffi_boundary/call_ffi_safe.md) |
