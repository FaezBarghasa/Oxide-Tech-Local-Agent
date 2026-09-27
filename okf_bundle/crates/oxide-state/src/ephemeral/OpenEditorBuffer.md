---
okf_version: "0.2"
type: Class
title: OpenEditorBuffer
description: An open editor buffer/tab in the ephemeral memory
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/OpenEditorBuffer
language: rust
---

# OpenEditorBuffer

An open editor buffer/tab in the ephemeral memory

## Signature

```rust
pub struct OpenEditorBuffer
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

An open editor buffer/tab in the ephemeral memory
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `file_path`
- `cursor_line`
- `cursor_col`
- `selection`
- `is_dirty`
- `last_accessed`

## Source
Lines 18–25 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
