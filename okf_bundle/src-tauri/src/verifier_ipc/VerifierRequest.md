---
okf_version: "0.2"
type: Class
title: VerifierRequest
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: src-tauri/src/verifier_ipc.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/verifier_ipc/VerifierRequest
language: rust
---

# VerifierRequest

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct VerifierRequest
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

- `workspace`
- `task_id`
- `export_path`

## Source
Lines 8–13 in `src-tauri/src/verifier_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier_ipc](/src-tauri/src/verifier_ipc.md) |
