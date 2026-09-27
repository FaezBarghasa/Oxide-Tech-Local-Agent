---
okf_version: "0.2"
type: Function
title: evaluate_execution
description: "Comprehensive governance decision based on mode, tool risk, and arguments"
resource: crates/router/src/agent_modes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_modes/evaluate_execution
language: rust
---

# evaluate_execution

Comprehensive governance decision based on mode, tool risk, and arguments

## Signature

```rust
impl ToolPermissions { pub fn evaluate_execution(
        &self,
        mode: AgentMode,
        tool_name: &str,
        command_arg: Option<&str>,
        unattended: bool,
    ) -> PermissionDecision }
```

## Visibility

- `pub`

## Docstring

Comprehensive governance decision based on mode, tool risk, and arguments

## Source
Lines 309–392 in `crates/router/src/agent_modes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_modes](/crates/router/src/agent_modes.md) |
| calls | [classify_tool_call](/crates/router/src/agent_modes/classify_tool_call.md) |
