---
okf_version: "0.2"
type: Function
title: classify_tool_call
description: Classify a tool call into its intrinsic Risk Class
resource: crates/router/src/agent_modes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_modes/classify_tool_call
language: rust
---

# classify_tool_call

Classify a tool call into its intrinsic Risk Class

## Signature

```rust
pub fn classify_tool_call(tool_name: &str) -> RiskClass
```

## Visibility

- `pub`

## Docstring

Classify a tool call into its intrinsic Risk Class

## Source
Lines 143–193 in `crates/router/src/agent_modes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_modes](/crates/router/src/agent_modes.md) |
| called_by | [evaluate_execution](/crates/router/src/agent_modes/evaluate_execution.md) |
