---
okf_version: "0.2"
type: Class
title: ExecutionResult
description: The result of a sandboxed command execution.
resource: crates/sandbox/src/execution.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:sandbox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/sandbox/src/execution/ExecutionResult
language: rust
---

# ExecutionResult

The result of a sandboxed command execution.

## Signature

```rust
pub struct ExecutionResult
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

The result of a sandboxed command execution.
[derive(Debug, Clone)]

## Methods

- `exit_code`
- `stdout`
- `stderr`

## Source
Lines 55–59 in `crates/sandbox/src/execution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [execution](/crates/sandbox/src/execution.md) |
