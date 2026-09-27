---
okf_version: "0.2"
type: Function
title: execute_native
resource: crates/sandbox/src/execution.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:sandbox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/sandbox/src/execution/execute_native_1
language: rust
---

# execute_native

## Signature

```rust
fn execute_native(
        &self,
        cmd: &[&str],
        work_dir: &str,
    ) -> Result<ExecutionResult, String>
```

## Source
Lines 189–246 in `crates/sandbox/src/execution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [execution](/crates/sandbox/src/execution.md) |
