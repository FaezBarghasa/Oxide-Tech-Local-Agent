---
okf_version: "0.2"
type: Function
title: is_ready
description: Check if all dependencies for a node have passed.
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/is_ready_1
language: rust
---

# is_ready

Check if all dependencies for a node have passed.

## Signature

```rust
pub fn is_ready(&self, task_id: &str) -> bool
```

## Visibility

- `pub`

## Docstring

Check if all dependencies for a node have passed.

## Source
Lines 145–160 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
