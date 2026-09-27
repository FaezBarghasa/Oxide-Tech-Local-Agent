---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-network/src/acl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:47Z"
concept_id: crates/oxide-network/src/acl/new_1
language: rust
---

# new

## Signature

```rust
pub fn new(rules: Vec<AclRule>, default_action: AclAction) -> Result<Self, OxideError>
```

## Visibility

- `pub`

## Source
Lines 200–205 in `crates/oxide-network/src/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-network/src/acl.md) |
