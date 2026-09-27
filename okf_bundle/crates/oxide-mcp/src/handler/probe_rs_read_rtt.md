---
okf_version: "0.2"
type: Function
title: probe_rs_read_rtt
description: "[tool(description = \"Read RTT logs from target chip using probe-rs\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/probe_rs_read_rtt
language: rust
---

# probe_rs_read_rtt

[tool(description = "Read RTT logs from target chip using probe-rs")]

## Signature

```rust
impl McpServer { fn probe_rs_read_rtt(
        &self,
        Parameters(input): Parameters<ProbeRsReadRttInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Read RTT logs from target chip using probe-rs")]

## Source
Lines 531–540 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
