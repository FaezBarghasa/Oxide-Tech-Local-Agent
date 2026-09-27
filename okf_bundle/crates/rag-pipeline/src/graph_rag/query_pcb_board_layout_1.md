---
okf_version: "0.2"
type: Function
title: query_pcb_board_layout
description: Retrieve component context for a PCB project
resource: crates/rag-pipeline/src/graph_rag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:41:07Z"
concept_id: crates/rag-pipeline/src/graph_rag/query_pcb_board_layout_1
language: rust
---

# query_pcb_board_layout

Retrieve component context for a PCB project

## Signature

```rust
pub fn query_pcb_board_layout(
        &self,
        project_id: &str,
    ) -> Result<Vec<Value>, anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Retrieve component context for a PCB project

## Source
Lines 30–43 in `crates/rag-pipeline/src/graph_rag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graph_rag](/crates/rag-pipeline/src/graph_rag.md) |
