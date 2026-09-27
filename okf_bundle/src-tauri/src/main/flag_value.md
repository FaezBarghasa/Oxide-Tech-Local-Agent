---
okf_version: "0.2"
type: Function
title: flag_value
description: "Extract `--flag value` from a raw arg slice."
resource: src-tauri/src/main.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: src-tauri/src/main/flag_value
language: rust
---

# flag_value

Extract `--flag value` from a raw arg slice.

## Signature

```rust
fn flag_value(args: &[String], flag: &str) -> Option<String>
```

## Docstring

Extract `--flag value` from a raw arg slice.

## Source
Lines 51–53 in `src-tauri/src/main.rs`
