---
okf_version: "0.2"
type: Function
title: train
resource: crates/model-trainer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:59:29Z"
concept_id: crates/model-trainer/src/lib/train
language: rust
---

# train

## Signature

```rust
impl SubprocessTrainer { fn train(&self, req: TrainRequest) -> Result<AdapterBuild, TrainerError> }
```

## Source
Lines 183–215 in `crates/model-trainer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/model-trainer/src/lib.md) |
