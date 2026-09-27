---
okf_version: "0.2"
type: Function
title: evaluate
description: "Evaluates all features in the DAG in topological order, solving sketches and generating solids."
resource: crates/parametric-forge/src/evaluator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:parametric-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:58:23Z"
concept_id: crates/parametric-forge/src/evaluator/evaluate_1
language: rust
---

# evaluate

Evaluates all features in the DAG in topological order, solving sketches and generating solids.

## Signature

```rust
pub fn evaluate(&self, dag: &mut FeatureDAG) -> Result<Vec<EvaluatedSolid>, ParametricError>
```

## Visibility

- `pub`

## Docstring

Evaluates all features in the DAG in topological order, solving sketches and generating solids.

## Source
Lines 29–136 in `crates/parametric-forge/src/evaluator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [evaluator](/crates/parametric-forge/src/evaluator.md) |
