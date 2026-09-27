---
okf_version: "0.2"
type: Class
title: VerificationDelta
description: "[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]"
resource: crates/self-evolver/src/delta_harvester.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:45:26Z"
concept_id: crates/self-evolver/src/delta_harvester/VerificationDelta
language: rust
---

# VerificationDelta

[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]

## Signature

```rust
pub struct VerificationDelta
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, SurrealValue)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]

## Methods

- `prompt`
- `original_failed_code`
- `verified_fixed_code`
- `compiler_error_log`
- `diff_summary`
- `reward_score`
- `verification_engine`
- `domain`

## Source
Lines 9–18 in `crates/self-evolver/src/delta_harvester.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [delta_harvester](/crates/self-evolver/src/delta_harvester.md) |
