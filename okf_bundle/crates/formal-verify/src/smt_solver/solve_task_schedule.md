---
okf_version: "0.2"
type: Function
title: solve_task_schedule
description: SMT constraint formulation and deterministic EDF solver with mathematical proof of schedulability.
resource: crates/formal-verify/src/smt_solver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T16:28:38Z"
concept_id: crates/formal-verify/src/smt_solver/solve_task_schedule
language: rust
---

# solve_task_schedule

SMT constraint formulation and deterministic EDF solver with mathematical proof of schedulability.

## Signature

```rust
pub fn solve_task_schedule(
    tasks: &[TaskConstraint],
    _num_cores: usize,
) -> Result<VerifiedSchedule, VerifyError>
```

## Visibility

- `pub`

## Docstring

SMT constraint formulation and deterministic EDF solver with mathematical proof of schedulability.

## Source
Lines 30–63 in `crates/formal-verify/src/smt_solver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [smt_solver](/crates/formal-verify/src/smt_solver.md) |
| called_by | [test_impossible_deadline](/crates/formal-verify/tests/verify_tests/test_impossible_deadline.md) |
| called_by | [test_schedule_feasibility](/crates/formal-verify/tests/verify_tests/test_schedule_feasibility.md) |
