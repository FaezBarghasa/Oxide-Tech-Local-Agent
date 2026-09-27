---
okf_version: "0.2"
type: Function
title: archive_to_bytes
description: Archive current state into a zero-copy byte buffer for microsecond cloning.
resource: crates/crucible/src/shadow_state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:crucible"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:55:39Z"
concept_id: crates/crucible/src/shadow_state/archive_to_bytes_1
language: rust
---

# archive_to_bytes

Archive current state into a zero-copy byte buffer for microsecond cloning.

## Signature

```rust
pub fn archive_to_bytes(&self) -> Result<Vec<u8>, String>
```

## Decorators

- `inline(always)`

## Visibility

- `pub`

## Docstring

Archive current state into a zero-copy byte buffer for microsecond cloning.
[inline(always)]

## Source
Lines 26–30 in `crates/crucible/src/shadow_state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [shadow_state](/crates/crucible/src/shadow_state.md) |
