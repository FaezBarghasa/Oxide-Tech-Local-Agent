---
okf_version: "0.2"
type: Function
title: query_ast_netlist_coupling
description: Query hardware-software cross-domain coupling (AST symbol mapped to Netlist component pin / peripheral)
resource: crates/rag-pipeline/src/graph_rag.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:41:07Z"
concept_id: crates/rag-pipeline/src/graph_rag/query_ast_netlist_coupling_1
language: rust
---

# query_ast_netlist_coupling

Query hardware-software cross-domain coupling (AST symbol mapped to Netlist component pin / peripheral)

## Signature

```rust
pub fn query_ast_netlist_coupling(
        &self,
        symbol_name: &str,
    ) -> Result<Vec<Value>, anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Query hardware-software cross-domain coupling (AST symbol mapped to Netlist component pin / peripheral)

## Source
Lines 46–59 in `crates/rag-pipeline/src/graph_rag.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graph_rag](/crates/rag-pipeline/src/graph_rag.md) |
