---
okf_version: "0.2"
type: Function
title: system_prompt_directive
description: Returns the system prompt modifier customized for the operational mode.
resource: crates/router/src/agent_modes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_modes/system_prompt_directive
language: rust
---

# system_prompt_directive

Returns the system prompt modifier customized for the operational mode.

## Signature

```rust
impl AgentMode { pub fn system_prompt_directive(&self) -> &'static str }
```

## Visibility

- `pub`

## Docstring

Returns the system prompt modifier customized for the operational mode.

## Source
Lines 53–96 in `crates/router/src/agent_modes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_modes](/crates/router/src/agent_modes.md) |
