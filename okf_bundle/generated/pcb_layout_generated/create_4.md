---
okf_version: "0.2"
type: Function
title: create
description: "[allow(unused_mut)]"
resource: generated/pcb_layout_generated.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:generated"
  - "domain:pcb_layout_generated.rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-07-25T15:09:34Z"
concept_id: generated/pcb_layout_generated/create_4
language: rust
---

# create

[allow(unused_mut)]

## Signature

```rust
impl Layout<'a> { pub fn create(
    _fbb: &'mut_bldr mut ::flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args LayoutArgs<'args>
  ) -> ::flatbuffers::WIPOffset<Layout<'bldr>> }
```

## Type Parameters

- `'bldr: 'args`
- `'args: 'mut_bldr`
- `'mut_bldr`
- `A: ::flatbuffers::Allocator + 'bldr`

## Visibility

- `pub`

## Docstring

[allow(unused_mut)]

## Source
Lines 475–483 in `generated/pcb_layout_generated.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb_layout_generated](/generated/pcb_layout_generated.md) |
