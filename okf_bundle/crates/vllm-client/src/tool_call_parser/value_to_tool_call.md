---
okf_version: "0.2"
type: Function
title: value_to_tool_call
resource: crates/vllm-client/src/tool_call_parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:58:37Z"
concept_id: crates/vllm-client/src/tool_call_parser/value_to_tool_call
language: rust
---

# value_to_tool_call

## Signature

```rust
impl ToolCallParser { fn value_to_tool_call(val: &Value) -> Option<ToolCall> }
```

## Source
Lines 102–126 in `crates/vllm-client/src/tool_call_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool_call_parser](/crates/vllm-client/src/tool_call_parser.md) |
