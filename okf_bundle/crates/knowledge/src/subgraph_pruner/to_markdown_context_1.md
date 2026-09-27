---
okf_version: "0.2"
type: Function
title: to_markdown_context
description: "Format the pruned subgraph into clean, token-efficient Markdown context for the LLM"
resource: crates/knowledge/src/subgraph_pruner.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/subgraph_pruner/to_markdown_context_1
language: rust
---

# to_markdown_context

Format the pruned subgraph into clean, token-efficient Markdown context for the LLM

## Signature

```rust
pub fn to_markdown_context(&self) -> String
```

## Visibility

- `pub`

## Docstring

Format the pruned subgraph into clean, token-efficient Markdown context for the LLM

## Source
Lines 15–53 in `crates/knowledge/src/subgraph_pruner.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subgraph_pruner](/crates/knowledge/src/subgraph_pruner.md) |
