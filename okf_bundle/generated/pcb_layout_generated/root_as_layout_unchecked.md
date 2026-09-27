---
okf_version: "0.2"
type: Function
title: root_as_layout_unchecked
description: "[inline]"
resource: generated/pcb_layout_generated.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:generated"
  - "domain:pcb_layout_generated.rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-07-25T15:09:34Z"
concept_id: generated/pcb_layout_generated/root_as_layout_unchecked
language: rust
---

# root_as_layout_unchecked

[inline]

## Signature

```rust
pub fn root_as_layout_unchecked(buf: &[u8]) -> Layout<'_>
```

## Visibility

- `pub`

## Docstring

[inline]
Assumes, without verification, that a buffer of bytes contains a Layout and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid `Layout`.

## Source
Lines 614–616 in `generated/pcb_layout_generated.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_layout_generated](/generated/pcb_layout_generated.md) |
