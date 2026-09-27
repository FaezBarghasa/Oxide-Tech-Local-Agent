---
okf_version: "0.2"
type: Class
title: PreferenceSample
description: Preference Pair (Chosen vs. Rejected completions)
resource: crates/model-trainer/src/rl_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:47:41Z"
concept_id: crates/model-trainer/src/rl_engine/PreferenceSample
language: rust
---

# PreferenceSample

Preference Pair (Chosen vs. Rejected completions)

## Signature

```rust
pub struct PreferenceSample
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Preference Pair (Chosen vs. Rejected completions)
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `prompt`
- `chosen`
- `rejected`

## Source
Lines 5–9 in `crates/model-trainer/src/rl_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rl_engine](/crates/model-trainer/src/rl_engine.md) |
