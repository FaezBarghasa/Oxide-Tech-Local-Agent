---
okf_version: "0.2"
type: Function
title: decompile_function
resource: crates/re-forge/src/neural_decompiler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:15:58Z"
concept_id: crates/re-forge/src/neural_decompiler/decompile_function
language: rust
---

# decompile_function

## Signature

```rust
impl NeuralDecompiler { pub fn decompile_function(
        &self,
        func: &DisassembledFunction,
        cfg: &ControlFlowGraph,
    ) -> Result<DecompilationResult> }
```

## Visibility

- `pub`

## Source
Lines 30–108 in `crates/re-forge/src/neural_decompiler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [neural_decompiler](/crates/re-forge/src/neural_decompiler.md) |
