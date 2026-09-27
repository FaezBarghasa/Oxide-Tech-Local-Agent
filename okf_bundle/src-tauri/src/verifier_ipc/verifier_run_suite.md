---
okf_version: "0.2"
type: Function
title: verifier_run_suite
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
concept_id: src-tauri/src/verifier_ipc/verifier_run_suite
language: rust
---

# verifier_run_suite

[tauri::command]

## Signature

```rust
pub fn verifier_run_suite(request: VerifierRequest) -> Result<EvidenceBundleDto, String>
```

## Decorators

- `tauri::command`

## Visibility

- `pub`

## Docstring

[tauri::command]

## Source
Lines 121–126 in `src-tauri/src/verifier_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier_ipc](/src-tauri/src/verifier_ipc.md) |
| calls | [run_verification](/src-tauri/src/verifier_ipc/run_verification.md) |
