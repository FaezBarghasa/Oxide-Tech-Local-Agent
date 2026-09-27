---
okf_version: "0.2"
type: Function
title: extract_from_text
description: "Attempt to parse tool calls from various model outputs (JSON, markdown codeblocks, XML tags)."
resource: crates/vllm-client/src/tool_call_parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:58:37Z"
concept_id: crates/vllm-client/src/tool_call_parser/extract_from_text_1
language: rust
---

# extract_from_text

Attempt to parse tool calls from various model outputs (JSON, markdown codeblocks, XML tags).

## Signature

```rust
pub fn extract_from_text(content: &str) -> Vec<ToolCall>
```

## Visibility

- `pub`

## Docstring

Attempt to parse tool calls from various model outputs (JSON, markdown codeblocks, XML tags).

## Source
Lines 9–32 in `crates/vllm-client/src/tool_call_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool_call_parser](/crates/vllm-client/src/tool_call_parser.md) |
