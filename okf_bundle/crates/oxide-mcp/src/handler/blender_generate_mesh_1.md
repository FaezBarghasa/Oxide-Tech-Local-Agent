---
okf_version: "0.2"
type: Function
title: blender_generate_mesh
description: "[tool(description = \"Generate 3D mesh object in Blender\")]"
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/blender_generate_mesh_1
language: rust
---

# blender_generate_mesh

[tool(description = "Generate 3D mesh object in Blender")]

## Signature

```rust
fn blender_generate_mesh(
        &self,
        Parameters(input): Parameters<BlenderMeshInput>,
    ) -> Result<CallToolResult, McpError>
```

## Decorators

- `tool(description = "Generate 3D mesh object in Blender")`

## Docstring

[tool(description = "Generate 3D mesh object in Blender")]

## Source
Lines 590–602 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
