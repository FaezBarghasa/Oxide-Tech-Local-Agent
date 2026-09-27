---
okf_version: "0.2"
type: Function
title: pid
resource: crates/oxide-security/src/process_containment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/process_containment/pid
language: rust
---

# pid

## Signature

```rust
impl ProcessTreeGuard { pub fn pid(&self) -> u32 }
```

## Visibility

- `pub`

## Source
Lines 33–35 in `crates/oxide-security/src/process_containment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [process_containment](/crates/oxide-security/src/process_containment.md) |
