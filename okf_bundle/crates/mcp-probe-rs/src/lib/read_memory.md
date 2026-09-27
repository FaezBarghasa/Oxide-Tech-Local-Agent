---
okf_version: "0.2"
type: Function
title: read_memory
description: "[tool(description = \"Read a memory region via probe-rs (read‑only, auto‑exec)\")]"
resource: crates/mcp-probe-rs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/lib/read_memory
language: rust
---

# read_memory

[tool(description = "Read a memory region via probe-rs (read‑only, auto‑exec)")]

## Signature

```rust
impl ProbeRsServer { fn read_memory(
        &self,
        Parameters(input): Parameters<ReadMemoryInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Read a memory region via probe-rs (read‑only, auto‑exec)")]

## Source
Lines 117–153 in `crates/mcp-probe-rs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-probe-rs/src/lib.md) |
