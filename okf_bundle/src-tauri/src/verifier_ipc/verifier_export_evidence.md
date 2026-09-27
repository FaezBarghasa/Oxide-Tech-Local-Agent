---
okf_version: "0.2"
type: Function
title: verifier_export_evidence
description: "[tauri::command]"
resource: src-tauri/src/verifier_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/verifier_ipc/verifier_export_evidence
language: rust
---

# verifier_export_evidence

[tauri::command]

## Signature

```rust
pub fn verifier_export_evidence(
    bundle_dto: EvidenceBundleDto,
    target_dir: String,
) -> Result<String, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 129–157 in `src-tauri/src/verifier_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier_ipc](/src-tauri/src/verifier_ipc.md) |
