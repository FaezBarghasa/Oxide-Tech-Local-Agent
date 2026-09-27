---
okf_version: "0.2"
type: Class
title: ReforgeResult
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: src-tauri/src/reforge_ipc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/reforge_ipc/ReforgeResult
language: rust
---

# ReforgeResult

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct ReforgeResult
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `domain`
- `file_size`
- `format`
- `entry_point`
- `arm_vector_table`
- `rtos`
- `avg_entropy`
- `entropy_chunks`
- `ptx_analysis`
- `functions`
- `total_instructions`
- `decompiled_code`

## Source
Lines 72–85 in `src-tauri/src/reforge_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [reforge_ipc](/src-tauri/src/reforge_ipc.md) |
