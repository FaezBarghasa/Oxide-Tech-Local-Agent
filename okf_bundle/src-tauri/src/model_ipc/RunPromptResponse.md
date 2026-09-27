---
okf_version: "0.2"
type: Class
title: RunPromptResponse
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
concept_id: src-tauri/src/model_ipc/RunPromptResponse
language: rust
---

# RunPromptResponse

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct RunPromptResponse
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `text`
- `model`
- `provider`
- `tokens_used`
- `latency_ms`
- `error`

## Source
Lines 40–47 in `src-tauri/src/model_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [model_ipc](/src-tauri/src/model_ipc.md) |
