---
okf_version: "0.2"
type: Function
title: open
description: Memory map a GGUF / model file with OS virtual address mapping (zero heap-allocation copy)
resource: crates/model-trainer/src/distributed.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:53:31Z"
concept_id: crates/model-trainer/src/distributed/open_1
language: rust
---

# open

Memory map a GGUF / model file with OS virtual address mapping (zero heap-allocation copy)

## Signature

```rust
pub fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error>
```

## Visibility

- `pub`

## Docstring

Memory map a GGUF / model file with OS virtual address mapping (zero heap-allocation copy)

## Source
Lines 63–71 in `crates/model-trainer/src/distributed.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributed](/crates/model-trainer/src/distributed.md) |
