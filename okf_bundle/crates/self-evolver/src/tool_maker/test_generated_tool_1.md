---
okf_version: "0.2"
type: Function
title: test_generated_tool
resource: crates/self-evolver/src/tool_maker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/self-evolver/src/tool_maker/test_generated_tool_1
language: rust
---

# test_generated_tool

## Signature

```rust
pub fn test_generated_tool(
        &self,
        script_path: &Path,
        _sample_inputs: &serde_json::Value,
    ) -> Result<TestResult>
```

## Visibility

- `pub`

## Source
Lines 254–277 in `crates/self-evolver/src/tool_maker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool_maker](/crates/self-evolver/src/tool_maker.md) |
