---
okf_version: "0.2"
type: Function
title: call_ffi_safe
description: Execute a foreign function or native operation behind a panic-isolated boundary fence.
resource: crates/oxide-core/src/ffi_boundary.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/ffi_boundary/call_ffi_safe
language: rust
---

# call_ffi_safe

Execute a foreign function or native operation behind a panic-isolated boundary fence.

## Signature

```rust
pub fn call_ffi_safe(operation_name: &str, f: F) -> Result<R, OxideError>
```

## Type Parameters

- `F`
- `R`

## Visibility

- `pub`

## Docstring

Execute a foreign function or native operation behind a panic-isolated boundary fence.

## Source
Lines 5–33 in `crates/oxide-core/src/ffi_boundary.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ffi_boundary](/crates/oxide-core/src/ffi_boundary.md) |
| called_by | [call_symbol](/crates/oxide-core/src/dynamic_loader/call_symbol.md) |
| called_by | [test_call_ffi_safe_panic_isolation](/crates/oxide-core/src/ffi_boundary/test_call_ffi_safe_panic_isolation.md) |
| called_by | [test_call_ffi_safe_success](/crates/oxide-core/src/ffi_boundary/test_call_ffi_safe_success.md) |
