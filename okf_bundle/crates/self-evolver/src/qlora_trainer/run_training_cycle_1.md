---
okf_version: "0.2"
type: Function
title: run_training_cycle
description: Dispatches the training job to the local fine-tuning runner (SGLang/Unsloth or local torch script).
resource: crates/self-evolver/src/qlora_trainer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T11:46:02Z"
concept_id: crates/self-evolver/src/qlora_trainer/run_training_cycle_1
language: rust
---

# run_training_cycle

Dispatches the training job to the local fine-tuning runner (SGLang/Unsloth or local torch script).

## Signature

```rust
pub fn run_training_cycle(&self, dataset_path: &Path) -> Result<TrainingReport>
```

## Visibility

- `pub`

## Docstring

Dispatches the training job to the local fine-tuning runner (SGLang/Unsloth or local torch script).

## Source
Lines 79–109 in `crates/self-evolver/src/qlora_trainer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qlora_trainer](/crates/self-evolver/src/qlora_trainer.md) |
