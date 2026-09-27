---
okf_version: "0.2"
type: Function
title: decide
resource: crates/ratchet/src/gate.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:ratchet"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/ratchet/src/gate/decide
language: rust
---

# decide

## Signature

```rust
impl PromotionGate { pub fn decide(&self, candidate: &MetricTuple, baseline: Option<&MetricTuple>) -> Verdict }
```

## Visibility

- `pub`

## Source
Lines 27–72 in `crates/ratchet/src/gate.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gate](/crates/ratchet/src/gate.md) |
