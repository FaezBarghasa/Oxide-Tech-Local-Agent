---
okf_version: "0.2"
type: Function
title: run_reforge_cli
resource: src-tauri/src/main.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: src-tauri/src/main/run_reforge_cli
language: rust
---

# run_reforge_cli

## Signature

```rust
fn run_reforge_cli(file_path: PathBuf, arch: String, summary: bool, decompile: bool, json: bool) -> Result<()>
```

## Source
Lines 174–249 in `src-tauri/src/main.rs`
