---
okf_version: "0.2"
type: Function
title: from_archived_bytes
description: Clone state from an archived byte slice with zero heap overhead.
resource: crates/crucible/src/shadow_state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:55:39Z"
concept_id: crates/crucible/src/shadow_state/from_archived_bytes
language: rust
---

# from_archived_bytes

Clone state from an archived byte slice with zero heap overhead.

## Signature

```rust
impl WorkspaceSnapshot { pub fn from_archived_bytes(bytes: &[u8]) -> Result<Self, String> }
```

## Visibility

- `pub`

## Docstring

Clone state from an archived byte slice with zero heap overhead.
[inline(always)]

## Source
Lines 34–40 in `crates/crucible/src/shadow_state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shadow_state](/crates/crucible/src/shadow_state.md) |
