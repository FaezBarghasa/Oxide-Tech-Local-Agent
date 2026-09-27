---
okf_version: "0.2"
type: Function
title: get_open_buffers
description: Retrieve active open editor files
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/get_open_buffers_1
language: rust
---

# get_open_buffers

Retrieve active open editor files

## Signature

```rust
pub fn get_open_buffers(&self) -> Vec<OpenEditorBuffer>
```

## Visibility

- `pub`

## Docstring

Retrieve active open editor files

## Source
Lines 127–130 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
