---
okf_version: "0.2"
type: Module
title: lib
description: "# Autoresearch Frozen Training Harness"
resource: targets/llm-pretraining-rs/harness/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:targets"
  - "domain:llm-pretraining-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: targets/llm-pretraining-rs/harness/src/lib
language: rust
---

# lib

# Autoresearch Frozen Training Harness

## Docstring

# Autoresearch Frozen Training Harness

Owns dataset sharding, tokenizer, training loop, evaluation on the sealed
validation split, and `HostOps` dispatch table.

## Relationships

| Type | Target |
|------|--------|
| related | [host_create_model](/targets/llm-pretraining-rs/harness/src/lib/host_create_model.md) |
| related | [host_linear](/targets/llm-pretraining-rs/harness/src/lib/host_linear.md) |
| related | [host_rmsnorm](/targets/llm-pretraining-rs/harness/src/lib/host_rmsnorm.md) |
| related | [host_attention](/targets/llm-pretraining-rs/harness/src/lib/host_attention.md) |
| related | [host_mlp_swiglu](/targets/llm-pretraining-rs/harness/src/lib/host_mlp_swiglu.md) |
| related | [host_set_dtype](/targets/llm-pretraining-rs/harness/src/lib/host_set_dtype.md) |
| related | [host_create_adamw](/targets/llm-pretraining-rs/harness/src/lib/host_create_adamw.md) |
| related | [host_set_schedule_cosine](/targets/llm-pretraining-rs/harness/src/lib/host_set_schedule_cosine.md) |
| related | [host_adamw_step](/targets/llm-pretraining-rs/harness/src/lib/host_adamw_step.md) |
| related | [host_log_scalar](/targets/llm-pretraining-rs/harness/src/lib/host_log_scalar.md) |
| related | [host_log_grad_norms](/targets/llm-pretraining-rs/harness/src/lib/host_log_grad_norms.md) |
| related | [host_release_tensor](/targets/llm-pretraining-rs/harness/src/lib/host_release_tensor.md) |
| related | [host_release_model](/targets/llm-pretraining-rs/harness/src/lib/host_release_model.md) |
| related | [host_release_optim](/targets/llm-pretraining-rs/harness/src/lib/host_release_optim.md) |
| related | [compute_harness_digest](/targets/llm-pretraining-rs/harness/src/lib/compute_harness_digest.md) |
| related | [TrainingHarness](/targets/llm-pretraining-rs/harness/src/lib/TrainingHarness.md) |
| related | [default](/targets/llm-pretraining-rs/harness/src/lib/default.md) |
| related | [default](/targets/llm-pretraining-rs/harness/src/lib/default.md) |
| related | [run_training_loop](/targets/llm-pretraining-rs/harness/src/lib/run_training_loop.md) |
| related | [run_training_loop](/targets/llm-pretraining-rs/harness/src/lib/run_training_loop.md) |
| related | [test_harness_cdylib_hot_swap](/targets/llm-pretraining-rs/harness/src/lib/test_harness_cdylib_hot_swap.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
