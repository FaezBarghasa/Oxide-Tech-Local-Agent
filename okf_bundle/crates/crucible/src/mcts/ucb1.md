---
okf_version: "0.2"
type: Function
title: ucb1
description: "[inline(always)]"
resource: crates/crucible/src/mcts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T09:13:48Z"
concept_id: crates/crucible/src/mcts/ucb1
language: rust
---

# ucb1

[inline(always)]

## Signature

```rust
impl MctsBranch { pub fn ucb1(&self, total_parent_visits: usize, exploration_constant: f64) -> f64 }
```

## Visibility

- `pub`

## Docstring

[inline(always)]

## Source
Lines 21–29 in `crates/crucible/src/mcts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcts](/crates/crucible/src/mcts.md) |
