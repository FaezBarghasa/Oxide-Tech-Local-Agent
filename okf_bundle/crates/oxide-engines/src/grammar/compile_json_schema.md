---
okf_version: "0.2"
type: Function
title: compile_json_schema
description: Compile a JSON Schema into GBNF Grammar Rules
resource: crates/oxide-engines/src/grammar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:09:50Z"
concept_id: crates/oxide-engines/src/grammar/compile_json_schema
language: rust
---

# compile_json_schema

Compile a JSON Schema into GBNF Grammar Rules

## Signature

```rust
impl GbnfCompiler { pub fn compile_json_schema(schema: &serde_json::Value) -> Self }
```

## Visibility

- `pub`

## Docstring

Compile a JSON Schema into GBNF Grammar Rules

## Source
Lines 37–80 in `crates/oxide-engines/src/grammar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grammar](/crates/oxide-engines/src/grammar.md) |
