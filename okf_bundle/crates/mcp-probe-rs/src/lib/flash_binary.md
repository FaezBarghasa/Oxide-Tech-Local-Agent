---
okf_version: "0.2"
type: Function
title: flash_binary
description: "[tool(description = \"Flash a binary to the target chip (requires human confirmation)\")]"
resource: crates/mcp-probe-rs/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/lib/flash_binary
language: rust
---

# flash_binary

[tool(description = "Flash a binary to the target chip (requires human confirmation)")]

## Signature

```rust
impl ProbeRsServer { fn flash_binary(
        &self,
        Parameters(input): Parameters<FlashBinaryInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Flash a binary to the target chip (requires human confirmation)")]

## Source
Lines 181–219 in `crates/mcp-probe-rs/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-probe-rs/src/lib.md) |
