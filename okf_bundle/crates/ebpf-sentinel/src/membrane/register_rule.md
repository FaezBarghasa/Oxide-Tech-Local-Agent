---
okf_version: "0.2"
type: Function
title: register_rule
description: Add an active constitutional rule
resource: crates/ebpf-sentinel/src/membrane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:ebpf-sentinel"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:54:15Z"
concept_id: crates/ebpf-sentinel/src/membrane/register_rule
language: rust
---

# register_rule

Add an active constitutional rule

## Signature

```rust
impl ConstitutionalMembrane { pub fn register_rule(&self, rule: ConstitutionalRule) }
```

## Visibility

- `pub`

## Docstring

Add an active constitutional rule

## Source
Lines 26–29 in `crates/ebpf-sentinel/src/membrane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [membrane](/crates/ebpf-sentinel/src/membrane.md) |
