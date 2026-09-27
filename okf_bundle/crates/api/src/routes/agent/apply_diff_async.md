---
okf_version: "0.2"
type: Function
title: apply_diff_async
resource: crates/api/src/routes/agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: crates/api/src/routes/agent/apply_diff_async
language: rust
---

# apply_diff_async

## Signature

```rust
fn apply_diff_async(
    workspace: &str,
    filepath: &str,
    search: &str,
    replace: &str,
) -> Result<(), String>
```

## Source
Lines 88–109 in `crates/api/src/routes/agent.rs`
