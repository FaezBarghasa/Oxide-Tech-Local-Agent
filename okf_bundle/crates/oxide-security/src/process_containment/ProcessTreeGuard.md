---
okf_version: "0.2"
type: Class
title: ProcessTreeGuard
description: Process Tree Guard tracking child processes and process groups to guarantee zero-zombie teardown
resource: crates/oxide-security/src/process_containment.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/process_containment/ProcessTreeGuard
language: rust
---

# ProcessTreeGuard

Process Tree Guard tracking child processes and process groups to guarantee zero-zombie teardown

## Signature

```rust
pub struct ProcessTreeGuard
```

## Visibility

- `pub`

## Docstring

Process Tree Guard tracking child processes and process groups to guarantee zero-zombie teardown

## Methods

- `pgid`
- `pid`
- `grace_period`

## Source
Lines 13–17 in `crates/oxide-security/src/process_containment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [process_containment](/crates/oxide-security/src/process_containment.md) |
