---
okf_version: "0.2"
type: Function
title: calculate_impact_from_graph
resource: crates/optio/src/impact_analysis.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/optio/src/impact_analysis/calculate_impact_from_graph_1
language: rust
---

# calculate_impact_from_graph

## Signature

```rust
pub fn calculate_impact_from_graph(
        modified_fn_id: &str,
        caller_map: &std::collections::HashMap<String, Vec<(String, String)>>, // node_id -> Vec<(caller_id, file_path)>
    ) -> ImpactSurface
```

## Visibility

- `pub`

## Source
Lines 15–51 in `crates/optio/src/impact_analysis.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impact_analysis](/crates/optio/src/impact_analysis.md) |
