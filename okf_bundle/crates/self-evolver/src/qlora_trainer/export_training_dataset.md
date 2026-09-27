---
okf_version: "0.2"
type: Function
title: export_training_dataset
description: Prepares training dataset in JSONL format from collected verification deltas.
resource: crates/self-evolver/src/qlora_trainer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T11:46:02Z"
concept_id: crates/self-evolver/src/qlora_trainer/export_training_dataset
language: rust
---

# export_training_dataset

Prepares training dataset in JSONL format from collected verification deltas.

## Signature

```rust
impl QLoraTrainer { pub fn export_training_dataset(
        &self,
        deltas: &[T],
        dataset_name: &str,
    ) -> Result<PathBuf> }
```

## Type Parameters

- `T: Serialize`

## Visibility

- `pub`

## Docstring

Prepares training dataset in JSONL format from collected verification deltas.

## Source
Lines 58–76 in `crates/self-evolver/src/qlora_trainer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [qlora_trainer](/crates/self-evolver/src/qlora_trainer.md) |
