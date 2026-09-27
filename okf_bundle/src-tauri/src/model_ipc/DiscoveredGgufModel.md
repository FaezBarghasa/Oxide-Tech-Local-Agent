---
okf_version: "0.2"
type: Class
title: DiscoveredGgufModel
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: src-tauri/src/model_ipc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:27:16Z"
concept_id: src-tauri/src/model_ipc/DiscoveredGgufModel
language: rust
---

# DiscoveredGgufModel

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct DiscoveredGgufModel
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`
- `serde(rename_all = "camelCase")`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]
[serde(rename_all = "camelCase")]

## Methods

- `path`
- `name`
- `size_gb`
- `version`
- `tensors`
- `metadata_entries`

## Source
Lines 598–605 in `src-tauri/src/model_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [model_ipc](/src-tauri/src/model_ipc.md) |
