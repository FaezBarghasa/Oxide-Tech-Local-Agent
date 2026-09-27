---
okf_version: "0.2"
type: Function
title: evaluate_candidates
description: Parallel simulation of candidate actions across Rayon threads using zero-copy state branches.
resource: crates/crucible/src/mcts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:13:48Z"
concept_id: crates/crucible/src/mcts/evaluate_candidates
language: rust
---

# evaluate_candidates

Parallel simulation of candidate actions across Rayon threads using zero-copy state branches.

## Signature

```rust
impl MctsDecisionEngine { pub fn evaluate_candidates(
        &self,
        base_state: &WorkspaceSnapshot,
        candidate_actions: &[String],
        evaluation_fn: F,
    ) -> Option<(String, f64)> }
```

## Type Parameters

- `F`

## Visibility

- `pub`

## Docstring

Parallel simulation of candidate actions across Rayon threads using zero-copy state branches.

## Source
Lines 52–77 in `crates/crucible/src/mcts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcts](/crates/crucible/src/mcts.md) |
