---
okf_version: "0.2"
type: Function
title: synthesize_and_verify_tool
description: "Synthesize a missing MCP tool, write it to disk, and verify it in a sandbox"
resource: crates/self-evolver/src/tool_maker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/self-evolver/src/tool_maker/synthesize_and_verify_tool_1
language: rust
---

# synthesize_and_verify_tool

Synthesize a missing MCP tool, write it to disk, and verify it in a sandbox

## Signature

```rust
pub fn synthesize_and_verify_tool(
        &self,
        task_gap_description: &str,
        sample_inputs: serde_json::Value,
    ) -> Result<ToolSpecification>
```

## Visibility

- `pub`

## Docstring

Synthesize a missing MCP tool, write it to disk, and verify it in a sandbox

## Source
Lines 56–110 in `crates/self-evolver/src/tool_maker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool_maker](/crates/self-evolver/src/tool_maker.md) |
