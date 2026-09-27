---
okf_version: "0.2"
type: Function
title: execute
resource: crates/sandbox/src/execution.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:sandbox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/sandbox/src/execution/execute
language: rust
---

# execute

## Signature

```rust
impl SandboxSpec { pub fn execute(&self, cmd: &[&str], work_dir: &str) -> Result<ExecutionResult, String> }
```

## Visibility

- `pub`

## Source
Lines 62–99 in `crates/sandbox/src/execution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [execution](/crates/sandbox/src/execution.md) |
