---
okf_version: "0.2"
type: Function
title: new
resource: crates/crucible/src/shadow_state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:55:39Z"
concept_id: crates/crucible/src/shadow_state/new
language: rust
---

# new

## Signature

```rust
impl WorkspaceSnapshot { pub fn new(task_id: impl Into<String>, step_index: usize) -> Self }
```

## Visibility

- `pub`

## Source
Lines 14–22 in `crates/crucible/src/shadow_state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shadow_state](/crates/crucible/src/shadow_state.md) |
