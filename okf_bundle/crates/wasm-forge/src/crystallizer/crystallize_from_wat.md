---
okf_version: "0.2"
type: Function
title: crystallize_from_wat
description: Crystallize a verified recurring tool sequence into an immutable WebAssembly module.
resource: crates/wasm-forge/src/crystallizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:wasm-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/wasm-forge/src/crystallizer/crystallize_from_wat
language: rust
---

# crystallize_from_wat

Crystallize a verified recurring tool sequence into an immutable WebAssembly module.

## Signature

```rust
impl SkillCrystallizer { pub fn crystallize_from_wat(
        &self,
        name: &str,
        description: &str,
        wat_source: &str,
    ) -> Result<CrystallizedTool, String> }
```

## Visibility

- `pub`

## Docstring

Crystallize a verified recurring tool sequence into an immutable WebAssembly module.

## Source
Lines 35–57 in `crates/wasm-forge/src/crystallizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crystallizer](/crates/wasm-forge/src/crystallizer.md) |
