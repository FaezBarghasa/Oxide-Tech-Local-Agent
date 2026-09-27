---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-protocol/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T14:30:29Z"
concept_id: crates/oxide-protocol/src/lib/new
language: rust
---

# new

## Signature

```rust
impl DtxRecord { pub fn new(
        title: impl Into<String>,
        initiator: impl Into<String>,
        domains: Vec<DomainTarget>,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 66–81 in `crates/oxide-protocol/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-protocol/src/lib.md) |
