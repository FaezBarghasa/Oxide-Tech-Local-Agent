---
okf_version: "0.2"
type: Class
title: TieredCacheMetrics
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
concept_id: src-tauri/src/model_ipc/TieredCacheMetrics
language: rust
---

# TieredCacheMetrics

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct TieredCacheMetrics
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

- `pinned_vram_pages`
- `ddr5_host_pages`
- `total_mappings`
- `compaction_active`

## Source
Lines 619–624 in `src-tauri/src/model_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [model_ipc](/src-tauri/src/model_ipc.md) |
