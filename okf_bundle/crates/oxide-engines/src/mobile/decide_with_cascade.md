---
okf_version: "0.2"
type: Function
title: decide_with_cascade
resource: crates/oxide-engines/src/mobile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:53:11Z"
concept_id: crates/oxide-engines/src/mobile/decide_with_cascade
language: rust
---

# decide_with_cascade

## Signature

```rust
impl MobileDecisionEngine { pub fn decide_with_cascade(
        &self,
        state: String,
        criteria: String,
        candidates: Vec<String>,
        confidence_spread_threshold: f32,
        min_confidence_threshold: f32,
    ) -> Result<DecisionOutput, String> }
```

## Visibility

- `pub`

## Source
Lines 41–60 in `crates/oxide-engines/src/mobile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile](/crates/oxide-engines/src/mobile.md) |
