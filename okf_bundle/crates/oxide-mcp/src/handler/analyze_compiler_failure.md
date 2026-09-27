---
okf_version: "0.2"
type: Function
title: analyze_compiler_failure
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/analyze_compiler_failure
language: rust
---

# analyze_compiler_failure

## Signature

```rust
impl McpServer { fn analyze_compiler_failure(
        &self,
        Parameters(input): Parameters<AnalyzeCompilerFailureInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Source
Lines 169–213 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
