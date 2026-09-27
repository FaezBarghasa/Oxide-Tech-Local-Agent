---
okf_version: "0.2"
type: Class
title: TopologyErrorMap
description: Structured topology error report for ReAct agent self-correction.
resource: crates/scene-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/verifier/TopologyErrorMap
language: rust
---

# TopologyErrorMap

Structured topology error report for ReAct agent self-correction.

## Signature

```rust
pub struct TopologyErrorMap
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Structured topology error report for ReAct agent self-correction.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `rule`
- `severity`
- `message`
- `problematic_edges`

## Source
Lines 31–36 in `crates/scene-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/scene-forge/src/verifier.md) |
