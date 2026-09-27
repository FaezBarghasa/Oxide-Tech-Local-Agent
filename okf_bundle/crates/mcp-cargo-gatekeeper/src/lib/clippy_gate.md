---
okf_version: "0.2"
type: Function
title: clippy_gate
description: "[tool(description = \"Run cargo clippy with -D warnings inside the sandbox\")]"
resource: crates/mcp-cargo-gatekeeper/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-cargo-gatekeeper"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:12Z"
concept_id: crates/mcp-cargo-gatekeeper/src/lib/clippy_gate
language: rust
---

# clippy_gate

[tool(description = "Run cargo clippy with -D warnings inside the sandbox")]

## Signature

```rust
impl CargoGatekeeperServer { fn clippy_gate(
        &self,
        Parameters(input): Parameters<ClippyInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Run cargo clippy with -D warnings inside the sandbox")]

## Source
Lines 55–82 in `crates/mcp-cargo-gatekeeper/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-cargo-gatekeeper/src/lib.md) |
