---
okf_version: "0.2"
type: Function
title: is_consequential
description: Returns true if the tool call carries consequential side effects
resource: crates/router/src/agent_modes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_modes/is_consequential_1
language: rust
---

# is_consequential

Returns true if the tool call carries consequential side effects

## Signature

```rust
pub fn is_consequential(&self) -> bool
```

## Visibility

- `pub`

## Docstring

Returns true if the tool call carries consequential side effects

## Source
Lines 137–139 in `crates/router/src/agent_modes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_modes](/crates/router/src/agent_modes.md) |
