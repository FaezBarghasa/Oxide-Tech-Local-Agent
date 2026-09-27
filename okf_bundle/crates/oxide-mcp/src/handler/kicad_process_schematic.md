---
okf_version: "0.2"
type: Function
title: kicad_process_schematic
description: "[tool(description = \"Generate schematic and run Design Rule Checks in KiCad\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/kicad_process_schematic
language: rust
---

# kicad_process_schematic

[tool(description = "Generate schematic and run Design Rule Checks in KiCad")]

## Signature

```rust
impl McpServer { fn kicad_process_schematic(
        &self,
        Parameters(input): Parameters<KiCadSchematicInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Docstring

[tool(description = "Generate schematic and run Design Rule Checks in KiCad")]

## Source
Lines 579–587 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
