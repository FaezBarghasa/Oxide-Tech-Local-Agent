---
okf_version: "0.2"
type: Function
title: test_schedule_feasibility
description: "[test]"
resource: crates/formal-verify/tests/verify_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/formal-verify/tests/verify_tests/test_schedule_feasibility
language: rust
---

# test_schedule_feasibility

[test]

## Signature

```rust
fn test_schedule_feasibility()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 8–29 in `crates/formal-verify/tests/verify_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verify_tests](/crates/formal-verify/tests/verify_tests.md) |
| calls | [solve_task_schedule](/crates/formal-verify/src/smt_solver/solve_task_schedule.md) |
