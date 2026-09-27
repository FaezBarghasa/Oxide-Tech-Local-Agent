---
okf_version: "0.2"
type: Function
title: disassemble_cubin
resource: crates/re-forge/src/sass_adapter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/re-forge/src/sass_adapter/disassemble_cubin
language: rust
---

# disassemble_cubin

## Signature

```rust
impl SassAdapter { pub fn disassemble_cubin(
        &self,
        cubin_path: &Path,
    ) -> Result<SassDisassembly, SassAdapterError> }
```

## Visibility

- `pub`

## Source
Lines 48–135 in `crates/re-forge/src/sass_adapter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sass_adapter](/crates/re-forge/src/sass_adapter.md) |
