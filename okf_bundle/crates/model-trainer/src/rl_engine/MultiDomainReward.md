---
okf_version: "0.2"
type: Class
title: MultiDomainReward
description: Multi-domain objective reward weights and evaluation for GRPO
resource: crates/model-trainer/src/rl_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:47:41Z"
concept_id: crates/model-trainer/src/rl_engine/MultiDomainReward
language: rust
---

# MultiDomainReward

Multi-domain objective reward weights and evaluation for GRPO

## Signature

```rust
pub struct MultiDomainReward
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Multi-domain objective reward weights and evaluation for GRPO
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `compile_weight`
- `formal_verify_weight`
- `drc_pass_weight`
- `sim_continuity_weight`

## Source
Lines 162–167 in `crates/model-trainer/src/rl_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rl_engine](/crates/model-trainer/src/rl_engine.md) |
