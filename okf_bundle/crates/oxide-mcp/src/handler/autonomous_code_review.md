---
okf_version: "0.2"
type: Function
title: autonomous_code_review
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/autonomous_code_review
language: rust
---

# autonomous_code_review

## Signature

```rust
impl McpServer { fn autonomous_code_review(
        &self,
        Parameters(input): Parameters<AutonomousCodeReviewInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Source
Lines 218–266 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
