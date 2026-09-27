---
okf_version: "0.2"
type: Function
title: disassemble_sass
description: Disassembles SASS using nvdisasm
resource: crates/re-forge/src/cuda/analyzer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/re-forge/src/cuda/analyzer/disassemble_sass
language: rust
---

# disassemble_sass

Disassembles SASS using nvdisasm

## Signature

```rust
impl CudaAnalyzer { pub fn disassemble_sass(&self, cubin_path: &str) -> Result<String> }
```

## Visibility

- `pub`

## Docstring

Disassembles SASS using nvdisasm

## Source
Lines 108–117 in `crates/re-forge/src/cuda/analyzer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [analyzer](/crates/re-forge/src/cuda/analyzer.md) |
