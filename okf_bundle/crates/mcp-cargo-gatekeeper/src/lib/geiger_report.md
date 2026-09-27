---
okf_version: "0.2"
type: Function
title: geiger_report
description: "[tool(description = \"Run cargo geiger to report unsafe usage\")]"
resource: crates/mcp-cargo-gatekeeper/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-cargo-gatekeeper"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:12Z"
concept_id: crates/mcp-cargo-gatekeeper/src/lib/geiger_report
language: rust
---

# geiger_report

[tool(description = "Run cargo geiger to report unsafe usage")]

## Signature

```rust
impl CargoGatekeeperServer { fn geiger_report(
        &self,
        Parameters(input): Parameters<GeigerInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Run cargo geiger to report unsafe usage")]

## Source
Lines 85–112 in `crates/mcp-cargo-gatekeeper/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-cargo-gatekeeper/src/lib.md) |
