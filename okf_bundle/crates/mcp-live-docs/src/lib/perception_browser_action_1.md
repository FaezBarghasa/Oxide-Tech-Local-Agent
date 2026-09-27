---
okf_version: "0.2"
type: Function
title: perception_browser_action
description: "[tool(description = \"Execute local interactive browser action via PinchTab daemon\")]"
resource: crates/mcp-live-docs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/mcp-live-docs/src/lib/perception_browser_action_1
language: rust
---

# perception_browser_action

[tool(description = "Execute local interactive browser action via PinchTab daemon")]

## Signature

```rust
fn perception_browser_action(
        &self,
        Parameters(input): Parameters<BrowserActionInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Execute local interactive browser action via PinchTab daemon")`

## Docstring

[tool(description = "Execute local interactive browser action via PinchTab daemon")]

## Source
Lines 157–179 in `crates/mcp-live-docs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-live-docs/src/lib.md) |
