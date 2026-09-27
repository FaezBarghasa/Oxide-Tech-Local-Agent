---
okf_version: "0.2"
type: Function
title: load
description: Load the target GGUF file with automatic IMatrix tensor decoding.
resource: crates/oxide-engines/src/mistral_rs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-engines/src/mistral_rs/load
language: rust
---

# load

Load the target GGUF file with automatic IMatrix tensor decoding.

## Signature

```rust
impl MistralRsProvider { pub fn load(&self) -> Result<(), OxideError> }
```

## Visibility

- `pub`

## Docstring

Load the target GGUF file with automatic IMatrix tensor decoding.

## Source
Lines 25–48 in `crates/oxide-engines/src/mistral_rs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mistral_rs](/crates/oxide-engines/src/mistral_rs.md) |
