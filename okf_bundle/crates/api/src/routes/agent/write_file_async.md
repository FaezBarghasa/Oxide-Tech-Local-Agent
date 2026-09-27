---
okf_version: "0.2"
type: Function
title: write_file_async
description: ── File tool helpers (always non-blocking) ───────────────────────────────────
resource: crates/api/src/routes/agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: crates/api/src/routes/agent/write_file_async
language: rust
---

# write_file_async

── File tool helpers (always non-blocking) ───────────────────────────────────

## Signature

```rust
fn write_file_async(workspace: &str, filepath: &str, content: &str) -> Result<(), String>
```

## Docstring

── File tool helpers (always non-blocking) ───────────────────────────────────

## Source
Lines 73–86 in `crates/api/src/routes/agent.rs`
