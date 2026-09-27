---
okf_version: "0.2"
type: Class
title: Critique
description: Critique feedback produced by the Critic persona.
resource: crates/optio/src/critic.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T11:45:13Z"
concept_id: crates/optio/src/critic/Critique
language: rust
---

# Critique

Critique feedback produced by the Critic persona.

## Signature

```rust
pub struct Critique
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Critique feedback produced by the Critic persona.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `passed`
- `severity`
- `remarks`
- `suggested_revisions`

## Source
Lines 8–13 in `crates/optio/src/critic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [critic](/crates/optio/src/critic.md) |
