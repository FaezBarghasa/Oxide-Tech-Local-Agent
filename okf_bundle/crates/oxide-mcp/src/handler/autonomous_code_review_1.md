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
concept_id: crates/oxide-mcp/src/handler/autonomous_code_review_1
language: rust
---

# autonomous_code_review

## Signature

```rust
fn autonomous_code_review(
        &self,
        Parameters(input): Parameters<AutonomousCodeReviewInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(
        description = "Autonomous code review for safety, concurrency, memory barriers and no_std constraints using direct local LLM reasoning"
    )`

## Source
Lines 218–266 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
