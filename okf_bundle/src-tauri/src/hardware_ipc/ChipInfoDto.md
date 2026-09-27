---
okf_version: "0.2"
type: Class
title: ChipInfoDto
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: src-tauri/src/hardware_ipc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:19:13Z"
concept_id: src-tauri/src/hardware_ipc/ChipInfoDto
language: rust
---

# ChipInfoDto

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct ChipInfoDto
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

- `name`
- `part`
- `cores`
- `memory_regions`

## Source
Lines 42–47 in `src-tauri/src/hardware_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_ipc](/src-tauri/src/hardware_ipc.md) |
