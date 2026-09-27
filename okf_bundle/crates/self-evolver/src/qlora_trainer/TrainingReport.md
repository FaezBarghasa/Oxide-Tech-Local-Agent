---
okf_version: "0.2"
type: Class
title: TrainingReport
description: Metadata summary of a completed training cycle.
resource: crates/self-evolver/src/qlora_trainer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T11:46:02Z"
concept_id: crates/self-evolver/src/qlora_trainer/TrainingReport
language: rust
---

# TrainingReport

Metadata summary of a completed training cycle.

## Signature

```rust
pub struct TrainingReport
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Metadata summary of a completed training cycle.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `adapter_path`
- `delta_count`
- `train_loss`
- `eval_passed`
- `timestamp`

## Source
Lines 35–41 in `crates/self-evolver/src/qlora_trainer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qlora_trainer](/crates/self-evolver/src/qlora_trainer.md) |
