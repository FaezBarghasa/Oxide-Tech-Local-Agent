---
okf_version: "0.2"
type: Function
title: format_preference_pair
resource: crates/model-trainer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:59:29Z"
concept_id: crates/model-trainer/src/lib/format_preference_pair_1
language: rust
---

# format_preference_pair

## Signature

```rust
pub fn format_preference_pair(prompt: &str, chosen: &str, rejected: &str) -> serde_json::Value
```

## Visibility

- `pub`

## Source
Lines 153–159 in `crates/model-trainer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/model-trainer/src/lib.md) |
