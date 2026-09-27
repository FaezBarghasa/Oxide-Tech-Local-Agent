---
okf_version: "0.2"
type: Class
title: ActionabilityConfig
description: "Configuration parameters for Playwright's 5-condition Auto-Waiting algorithm."
resource: crates/web-forge/src/actionability.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:37Z"
concept_id: crates/web-forge/src/actionability/ActionabilityConfig
language: rust
---

# ActionabilityConfig

Configuration parameters for Playwright's 5-condition Auto-Waiting algorithm.

## Signature

```rust
pub struct ActionabilityConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Configuration parameters for Playwright's 5-condition Auto-Waiting algorithm.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `timeout`
- `poll_interval`
- `stability_samples`
- `stability_epsilon`
- `check_visible`
- `check_stable`
- `check_enabled`
- `check_uncovered`

## Source
Lines 99–108 in `crates/web-forge/src/actionability.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actionability](/crates/web-forge/src/actionability.md) |
