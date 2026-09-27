---
okf_version: "0.2"
type: Function
title: has_opaque_constructs
description: Check if a command string contains opaque expansions or unvetted redirection
resource: crates/router/src/agent_modes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_modes/has_opaque_constructs
language: rust
---

# has_opaque_constructs

Check if a command string contains opaque expansions or unvetted redirection

## Signature

```rust
impl OpaqueConstructGuard { pub fn has_opaque_constructs(command: &str) -> bool }
```

## Visibility

- `pub`

## Docstring

Check if a command string contains opaque expansions or unvetted redirection

## Source
Lines 225–232 in `crates/router/src/agent_modes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_modes](/crates/router/src/agent_modes.md) |
