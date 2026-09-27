---
okf_version: "0.2"
type: Function
title: terminate_all
description: "Clean teardown: sends SIGTERM to process group, waits up to grace_period, then forces SIGKILL"
resource: crates/oxide-security/src/process_containment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/process_containment/terminate_all
language: rust
---

# terminate_all

Clean teardown: sends SIGTERM to process group, waits up to grace_period, then forces SIGKILL

## Signature

```rust
impl ProcessTreeGuard { pub fn terminate_all(&self) -> Result<(), ProcessContainmentError> }
```

## Visibility

- `pub`

## Docstring

Clean teardown: sends SIGTERM to process group, waits up to grace_period, then forces SIGKILL

## Source
Lines 42–67 in `crates/oxide-security/src/process_containment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [process_containment](/crates/oxide-security/src/process_containment.md) |
