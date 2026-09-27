---
okf_version: "0.2"
type: Function
title: list_probes
description: "[tool(description = \"List connected probe‑rs devices (read‑only)\")]"
resource: crates/mcp-probe-rs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/lib/list_probes
language: rust
---

# list_probes

[tool(description = "List connected probe‑rs devices (read‑only)")]

## Signature

```rust
impl ProbeRsServer { fn list_probes(
        &self,
        _input: Parameters<EmptyInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "List connected probe‑rs devices (read‑only)")]

## Source
Lines 156–178 in `crates/mcp-probe-rs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-probe-rs/src/lib.md) |
