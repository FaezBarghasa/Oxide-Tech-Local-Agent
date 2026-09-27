---
okf_version: "0.2"
type: Class
title: ActiveStackTrace
description: A captured runtime stack trace or panic diagnostic
resource: crates/oxide-state/src/ephemeral.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/ephemeral/ActiveStackTrace
language: rust
---

# ActiveStackTrace

A captured runtime stack trace or panic diagnostic

## Signature

```rust
pub struct ActiveStackTrace
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

A captured runtime stack trace or panic diagnostic
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `trace_id`
- `error_type`
- `message`
- `frames`
- `timestamp`

## Source
Lines 29–35 in `crates/oxide-state/src/ephemeral.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ephemeral](/crates/oxide-state/src/ephemeral.md) |
