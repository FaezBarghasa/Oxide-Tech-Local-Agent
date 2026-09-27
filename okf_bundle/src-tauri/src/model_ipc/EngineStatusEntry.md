---
okf_version: "0.2"
type: Class
title: EngineStatusEntry
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
concept_id: src-tauri/src/model_ipc/EngineStatusEntry
language: rust
---

# EngineStatusEntry

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct EngineStatusEntry
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

- `engine`
- `port`
- `status`
- `latency_ms`
- `active_backend`

## Source
Lines 609–615 in `src-tauri/src/model_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [model_ipc](/src-tauri/src/model_ipc.md) |
