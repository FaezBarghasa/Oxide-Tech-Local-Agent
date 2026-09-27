---
okf_version: "0.2"
type: Function
title: format
description: "Format conversation history and active tool definitions into Ornith-1.5's ChatML template."
resource: crates/oxide-tooling/src/ornith_formatter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-tooling"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-tooling/src/ornith_formatter/format_1
language: rust
---

# format

Format conversation history and active tool definitions into Ornith-1.5's ChatML template.

## Signature

```rust
pub fn format(
        messages: &[ChatMessage],
        tools: Option<&[Value]>,
        custom_system_prompt: Option<&str>,
    ) -> String
```

## Visibility

- `pub`

## Docstring

Format conversation history and active tool definitions into Ornith-1.5's ChatML template.

## Source
Lines 16–65 in `crates/oxide-tooling/src/ornith_formatter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ornith_formatter](/crates/oxide-tooling/src/ornith_formatter.md) |
