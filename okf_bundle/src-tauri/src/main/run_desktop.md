---
okf_version: "0.2"
type: Function
title: run_desktop
resource: src-tauri/src/main.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:20:55Z"
concept_id: src-tauri/src/main/run_desktop
language: rust
---

# run_desktop

## Signature

```rust
fn run_desktop(config: Option<String>)
```

## Source
Lines 55–113 in `src-tauri/src/main.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [src](/src-tauri/src/main.md) |
| calls | [spawn_background](/src-tauri/src/gateway_rt/spawn_background.md) |
| called_by | [main](/src-tauri/src/main/main.md) |
