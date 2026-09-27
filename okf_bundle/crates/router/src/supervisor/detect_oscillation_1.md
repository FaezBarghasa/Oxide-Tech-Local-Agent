---
okf_version: "0.2"
type: Function
title: detect_oscillation
description: "Detect oscillation if the same error is seen N >= 3 times in a row."
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/detect_oscillation_1
language: rust
---

# detect_oscillation

Detect oscillation if the same error is seen N >= 3 times in a row.

## Signature

```rust
pub fn detect_oscillation(
        &mut self,
        dag_id: Uuid,
        task_id: &str,
        error_signature: &str,
    ) -> bool
```

## Visibility

- `pub`

## Docstring

Detect oscillation if the same error is seen N >= 3 times in a row.

## Source
Lines 356–381 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
