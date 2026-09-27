---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-security/src/process_containment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/process_containment/new
language: rust
---

# new

## Signature

```rust
impl ProcessTreeGuard { pub fn new(pid: u32, pgid: Option<u32>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 20–26 in `crates/oxide-security/src/process_containment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [process_containment](/crates/oxide-security/src/process_containment.md) |
