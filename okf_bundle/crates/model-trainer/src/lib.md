---
okf_version: "0.2"
type: Module
title: lib
description: "# Model Trainer"
resource: crates/model-trainer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:59:29Z"
concept_id: crates/model-trainer/src/lib
language: rust
---

# lib

# Model Trainer

## Docstring

# Model Trainer

Offline LoRA SFT/DPO and rollout-based GRPO model training pipeline.
Harvests verified trajectories from `agent-journal` where `VerificationDelta == Pass`,
prepares dataset formats, executes sandboxed training loops, and manages
adapter lifecycle with quarantine benchmarks and canary promotion.

## Relationships

| Type | Target |
|------|--------|
| related | [TrainerError](/crates/model-trainer/src/lib/TrainerError.md) |
| related | [GgufQuantType](/crates/model-trainer/src/lib/GgufQuantType.md) |
| related | [QLoraQuantMethod](/crates/model-trainer/src/lib/QLoraQuantMethod.md) |
| related | [QLoraConfig](/crates/model-trainer/src/lib/QLoraConfig.md) |
| related | [default](/crates/model-trainer/src/lib/default.md) |
| related | [default](/crates/model-trainer/src/lib/default.md) |
| related | [TrainKind](/crates/model-trainer/src/lib/TrainKind.md) |
| related | [TrainRequest](/crates/model-trainer/src/lib/TrainRequest.md) |
| related | [AdapterBuild](/crates/model-trainer/src/lib/AdapterBuild.md) |
| related | [AdapterStage](/crates/model-trainer/src/lib/AdapterStage.md) |
| related | [AdapterRegistration](/crates/model-trainer/src/lib/AdapterRegistration.md) |
| related | [Trainer](/crates/model-trainer/src/lib/Trainer.md) |
| related | [TrajectoryExporter](/crates/model-trainer/src/lib/TrajectoryExporter.md) |
| related | [format_sharegpt](/crates/model-trainer/src/lib/format_sharegpt.md) |
| related | [format_preference_pair](/crates/model-trainer/src/lib/format_preference_pair.md) |
| related | [format_sharegpt](/crates/model-trainer/src/lib/format_sharegpt.md) |
| related | [format_preference_pair](/crates/model-trainer/src/lib/format_preference_pair.md) |
| related | [SubprocessTrainer](/crates/model-trainer/src/lib/SubprocessTrainer.md) |
| related | [new](/crates/model-trainer/src/lib/new.md) |
| related | [new](/crates/model-trainer/src/lib/new.md) |
| related | [kind](/crates/model-trainer/src/lib/kind.md) |
| related | [train](/crates/model-trainer/src/lib/train.md) |
| related | [kind](/crates/model-trainer/src/lib/kind.md) |
| related | [train](/crates/model-trainer/src/lib/train.md) |
| related | [CandleTrainer](/crates/model-trainer/src/lib/CandleTrainer.md) |
| related | [kind](/crates/model-trainer/src/lib/kind.md) |
| related | [train](/crates/model-trainer/src/lib/train.md) |
| related | [kind](/crates/model-trainer/src/lib/kind.md) |
| related | [train](/crates/model-trainer/src/lib/train.md) |
| related | [GrpoTrainer](/crates/model-trainer/src/lib/GrpoTrainer.md) |
| related | [kind](/crates/model-trainer/src/lib/kind.md) |
| related | [train](/crates/model-trainer/src/lib/train.md) |
| related | [kind](/crates/model-trainer/src/lib/kind.md) |
| related | [train](/crates/model-trainer/src/lib/train.md) |
| related | [QLoraTrainer](/crates/model-trainer/src/lib/QLoraTrainer.md) |
| related | [default](/crates/model-trainer/src/lib/default.md) |
| related | [default](/crates/model-trainer/src/lib/default.md) |
| related | [new](/crates/model-trainer/src/lib/new.md) |
| related | [export_gguf_lora_container](/crates/model-trainer/src/lib/export_gguf_lora_container.md) |
| related | [new](/crates/model-trainer/src/lib/new.md) |
| related | [export_gguf_lora_container](/crates/model-trainer/src/lib/export_gguf_lora_container.md) |
| related | [kind](/crates/model-trainer/src/lib/kind.md) |
| related | [train](/crates/model-trainer/src/lib/train.md) |
| related | [kind](/crates/model-trainer/src/lib/kind.md) |
| related | [train](/crates/model-trainer/src/lib/train.md) |
| related | [AdapterRegistry](/crates/model-trainer/src/lib/AdapterRegistry.md) |
| related | [default](/crates/model-trainer/src/lib/default.md) |
| related | [default](/crates/model-trainer/src/lib/default.md) |
| related | [new](/crates/model-trainer/src/lib/new.md) |
| related | [register_build](/crates/model-trainer/src/lib/register_build.md) |
| related | [evaluate_and_promote](/crates/model-trainer/src/lib/evaluate_and_promote.md) |
| related | [new](/crates/model-trainer/src/lib/new.md) |
| related | [register_build](/crates/model-trainer/src/lib/register_build.md) |
| related | [evaluate_and_promote](/crates/model-trainer/src/lib/evaluate_and_promote.md) |
| related | [test_adapter_training_and_promotion_cycle](/crates/model-trainer/src/lib/test_adapter_training_and_promotion_cycle.md) |
| related | [test_qlora_training_and_gguf_export](/crates/model-trainer/src/lib/test_qlora_training_and_gguf_export.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
