---
okf_version: "0.2"
type: Function
title: size_prefixed_root_as_layout_unchecked
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
concept_id: generated/pcb_layout_generated/size_prefixed_root_as_layout_unchecked
language: rust
---

# size_prefixed_root_as_layout_unchecked

[inline]

## Signature

```rust
pub fn size_prefixed_root_as_layout_unchecked(buf: &[u8]) -> Layout<'_>
```

## Visibility

- `pub`

## Docstring

[inline]
Assumes, without verification, that a buffer of bytes contains a size prefixed Layout and returns it.
# Safety
Callers must trust the given bytes do indeed contain a valid size prefixed `Layout`.

## Source
Lines 621–623 in `generated/pcb_layout_generated.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_layout_generated](/generated/pcb_layout_generated.md) |
