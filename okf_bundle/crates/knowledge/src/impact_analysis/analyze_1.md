---
okf_version: "0.2"
type: Function
title: analyze
description: Calculate downstream blast radius for a given modified node
resource: crates/knowledge/src/impact_analysis.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/impact_analysis/analyze_1
language: rust
---

# analyze

Calculate downstream blast radius for a given modified node

## Signature

```rust
pub fn analyze(graph: &MultiModalCodeGraph, target_id: &str) -> ImpactAnalysisReport
```

## Visibility

- `pub`

## Docstring

Calculate downstream blast radius for a given modified node

## Source
Lines 28–93 in `crates/knowledge/src/impact_analysis.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [impact_analysis](/crates/knowledge/src/impact_analysis.md) |
