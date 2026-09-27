---
okf_version: "0.2"
type: Class
title: DisassembledInstruction
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/re-forge/src/analyzer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/re-forge/src/analyzer/DisassembledInstruction
language: rust
---

# DisassembledInstruction

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct DisassembledInstruction
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `address`
- `mnemonic`
- `length`
- `is_branch`
- `is_call`
- `is_return`

## Source
Lines 19–26 in `crates/re-forge/src/analyzer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [analyzer](/crates/re-forge/src/analyzer.md) |
