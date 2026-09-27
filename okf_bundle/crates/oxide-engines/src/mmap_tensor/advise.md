---
okf_version: "0.2"
type: Function
title: advise
description: Advise the Linux kernel on paging strategies for this model region.
resource: crates/oxide-engines/src/mmap_tensor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:08:57Z"
concept_id: crates/oxide-engines/src/mmap_tensor/advise
language: rust
---

# advise

Advise the Linux kernel on paging strategies for this model region.

## Signature

```rust
impl MmapModel { pub fn advise(&self, advice: MemoryAdvice) -> Result<(), OxideError> }
```

## Visibility

- `pub`

## Docstring

Advise the Linux kernel on paging strategies for this model region.

## Source
Lines 99–125 in `crates/oxide-engines/src/mmap_tensor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mmap_tensor](/crates/oxide-engines/src/mmap_tensor.md) |
