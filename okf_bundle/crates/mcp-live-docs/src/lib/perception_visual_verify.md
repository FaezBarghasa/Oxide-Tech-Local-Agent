---
okf_version: "0.2"
type: Function
title: perception_visual_verify
resource: crates/mcp-live-docs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-live-docs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/mcp-live-docs/src/lib/perception_visual_verify
language: rust
---

# perception_visual_verify

## Signature

```rust
impl LiveDocsServer { fn perception_visual_verify(
        &self,
        Parameters(input): Parameters<VisualVerifyInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Source
Lines 184–202 in `crates/mcp-live-docs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-live-docs/src/lib.md) |
