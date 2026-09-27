---
okf_version: "0.2"
type: Function
title: evaluate
resource: crates/oxide-network/src/acl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:47Z"
concept_id: crates/oxide-network/src/acl/evaluate
language: rust
---

# evaluate

## Signature

```rust
impl AclEngine { pub fn evaluate(&self, meta: &PacketMeta) -> AclAction }
```

## Visibility

- `pub`

## Source
Lines 207–214 in `crates/oxide-network/src/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-network/src/acl.md) |
