---
okf_version: "0.2"
type: Function
title: record_verified_solution_with_reward
description: "[allow(clippy::too_many_arguments)]"
resource: crates/self-evolver/src/delta_harvester.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:45:26Z"
concept_id: crates/self-evolver/src/delta_harvester/record_verified_solution_with_reward
language: rust
---

# record_verified_solution_with_reward

[allow(clippy::too_many_arguments)]

## Signature

```rust
impl DeltaHarvester { pub fn record_verified_solution_with_reward(
        &self,
        prompt: &str,
        failed_code: &str,
        fixed_code: &str,
        compiler_log: &str,
        reward_score: f64,
        verification_engine: &str,
        domain: &str,
    ) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

[allow(clippy::too_many_arguments)]

## Source
Lines 49–87 in `crates/self-evolver/src/delta_harvester.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [delta_harvester](/crates/self-evolver/src/delta_harvester.md) |
