---
okf_version: "0.2"
type: Function
title: size_prefixed_root_as_layout
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
concept_id: generated/pcb_layout_generated/size_prefixed_root_as_layout
language: rust
---

# size_prefixed_root_as_layout

[inline]

## Signature

```rust
pub fn size_prefixed_root_as_layout(buf: &[u8]) -> Result<Layout<'_>, ::flatbuffers::InvalidFlatbuffer>
```

## Visibility

- `pub`

## Docstring

[inline]
Verifies that a buffer of bytes contains a size prefixed
`Layout` and returns it.
Note that verification is still experimental and may not
catch every error, or be maximally performant. For the
previous, unchecked, behavior use
`size_prefixed_root_as_layout_unchecked`.

## Source
Lines 581–583 in `generated/pcb_layout_generated.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_layout_generated](/generated/pcb_layout_generated.md) |
