---
okf_version: "0.2"
type: Function
title: update_editor_buffer
description: Update or register an open editor buffer state
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/update_editor_buffer_1
language: rust
---

# update_editor_buffer

Update or register an open editor buffer state

## Signature

```rust
pub fn update_editor_buffer(
        &self,
        file_path: &str,
        cursor_line: usize,
        cursor_col: usize,
        selection: Option<String>,
        is_dirty: bool,
    )
```

## Visibility

- `pub`

## Docstring

Update or register an open editor buffer state

## Source
Lines 78–103 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
