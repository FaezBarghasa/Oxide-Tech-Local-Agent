---
okf_version: "0.2"
type: Class
title: ToolPermissions
description: Granular tool use permissions
resource: crates/router/src/agent_modes.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_modes/ToolPermissions
language: rust
---

# ToolPermissions

Granular tool use permissions

## Signature

```rust
pub struct ToolPermissions
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Granular tool use permissions
[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]

## Methods

- `allow_read_files`
- `allow_write_files`
- `allow_terminal_exec`
- `allow_network_outbound`
- `allow_hardware_flash`
- `allow_git_commit`

## Source
Lines 265–272 in `crates/router/src/agent_modes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_modes](/crates/router/src/agent_modes.md) |
