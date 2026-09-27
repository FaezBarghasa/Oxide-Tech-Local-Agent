---
okf_version: "0.2"
type: Function
title: parse_markdown_json
resource: crates/vllm-client/src/tool_call_parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:58:37Z"
concept_id: crates/vllm-client/src/tool_call_parser/parse_markdown_json
language: rust
---

# parse_markdown_json

## Signature

```rust
impl ToolCallParser { fn parse_markdown_json(content: &str) -> Option<Vec<ToolCall>> }
```

## Source
Lines 60–90 in `crates/vllm-client/src/tool_call_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool_call_parser](/crates/vllm-client/src/tool_call_parser.md) |
