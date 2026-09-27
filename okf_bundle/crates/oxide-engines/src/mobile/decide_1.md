---
okf_version: "0.2"
type: Function
title: decide
resource: crates/oxide-engines/src/mobile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:53:11Z"
concept_id: crates/oxide-engines/src/mobile/decide_1
language: rust
---

# decide

## Signature

```rust
pub fn decide(
        &self,
        state: String,
        criteria: String,
        candidates: Vec<String>,
    ) -> Result<DecisionOutput, String>
```

## Visibility

- `pub`

## Source
Lines 27–39 in `crates/oxide-engines/src/mobile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile](/crates/oxide-engines/src/mobile.md) |
