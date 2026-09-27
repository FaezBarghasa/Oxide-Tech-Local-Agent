---
okf_version: "0.2"
type: Function
title: run_verification
resource: src-tauri/src/verifier_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: src-tauri/src/verifier_ipc/run_verification
language: rust
---

# run_verification

## Signature

```rust
pub fn run_verification(req: VerifierRequest) -> anyhow::Result<EvidenceBundleDto>
```

## Visibility

- `pub`

## Source
Lines 35–118 in `src-tauri/src/verifier_ipc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier_ipc](/src-tauri/src/verifier_ipc.md) |
| called_by | [run_verify_cli](/src-tauri/src/main/run_verify_cli.md) |
| called_by | [verifier_run_suite](/src-tauri/src/verifier_ipc/verifier_run_suite.md) |
