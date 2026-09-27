---
okf_version: "0.2"
type: Function
title: extract_tool_calls
description: Extract tool calls from an Ornith-1.5 raw model completion output.
resource: crates/oxide-tooling/src/ornith_formatter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tooling"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-tooling/src/ornith_formatter/extract_tool_calls
language: rust
---

# extract_tool_calls

Extract tool calls from an Ornith-1.5 raw model completion output.

## Signature

```rust
impl OrnithPromptFormatter { pub fn extract_tool_calls(completion: &str) -> Vec<ExtractedToolCall> }
```

## Visibility

- `pub`

## Docstring

Extract tool calls from an Ornith-1.5 raw model completion output.

## Source
Lines 68–92 in `crates/oxide-tooling/src/ornith_formatter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ornith_formatter](/crates/oxide-tooling/src/ornith_formatter.md) |
