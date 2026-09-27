---
okf_version: "0.2"
type: Function
title: has_dangerous_constructs
description: Check if a command includes dangerous execution/deletion flags or argument executors
resource: crates/router/src/agent_modes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_modes/has_dangerous_constructs_1
language: rust
---

# has_dangerous_constructs

Check if a command includes dangerous execution/deletion flags or argument executors

## Signature

```rust
pub fn has_dangerous_constructs(command: &str) -> bool
```

## Visibility

- `pub`

## Docstring

Check if a command includes dangerous execution/deletion flags or argument executors

## Source
Lines 235–252 in `crates/router/src/agent_modes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_modes](/crates/router/src/agent_modes.md) |
