---
okf_version: "0.2"
type: Function
title: run_doctor_cli
resource: src-tauri/src/main.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:20:55Z"
concept_id: src-tauri/src/main/run_doctor_cli
language: rust
---

# run_doctor_cli

## Signature

```rust
fn run_doctor_cli(json_output: bool)
```

## Source
Lines 121–172 in `src-tauri/src/main.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [src](/src-tauri/src/main.md) |
| calls | [run_diagnostics_scan](/src-tauri/src/doctor/run_diagnostics_scan.md) |
| called_by | [main](/src-tauri/src/main/main.md) |
