---
okf_version: "0.2"
type: Function
title: execute_in_sandbox
description: Convenience entry-point maintaining backwards compatibility
resource: crates/sandbox/src/execution.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:sandbox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/sandbox/src/execution/execute_in_sandbox
language: rust
---

# execute_in_sandbox

Convenience entry-point maintaining backwards compatibility

## Signature

```rust
pub fn execute_in_sandbox(cmd: &[&str], work_dir: &str) -> Result<ExecutionResult, String>
```

## Visibility

- `pub`

## Docstring

Convenience entry-point maintaining backwards compatibility

## Source
Lines 250–253 in `crates/sandbox/src/execution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [execution](/crates/sandbox/src/execution.md) |
