---
okf_version: "0.2"
type: Function
title: evaluate_and_promote
resource: crates/model-trainer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:59:29Z"
concept_id: crates/model-trainer/src/lib/evaluate_and_promote
language: rust
---

# evaluate_and_promote

## Signature

```rust
impl AdapterRegistry { pub fn evaluate_and_promote(
        &self,
        adapter_id: Uuid,
        benchmark_delta: f32,
        replay_equivalence: bool,
    ) -> Result<AdapterStage, TrainerError> }
```

## Visibility

- `pub`

## Source
Lines 462–505 in `crates/model-trainer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/model-trainer/src/lib.md) |
