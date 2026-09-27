---
okf_version: "0.2"
type: Function
title: read_register
description: "[tool(description = \"Read a register via probe-rs (read‑only, auto‑exec)\")]"
resource: crates/mcp-probe-rs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/lib/read_register
language: rust
---

# read_register

[tool(description = "Read a register via probe-rs (read‑only, auto‑exec)")]

## Signature

```rust
impl ProbeRsServer { fn read_register(
        &self,
        Parameters(input): Parameters<ReadRegisterInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Read a register via probe-rs (read‑only, auto‑exec)")]

## Source
Lines 87–114 in `crates/mcp-probe-rs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-probe-rs/src/lib.md) |
