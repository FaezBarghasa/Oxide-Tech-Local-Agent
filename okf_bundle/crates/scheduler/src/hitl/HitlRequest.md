---
okf_version: "0.2"
type: Class
title: HitlRequest
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/scheduler/src/hitl.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/hitl/HitlRequest
language: rust
---

# HitlRequest

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct HitlRequest
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `task_id`
- `tool_name`
- `risk_level`
- `description`
- `arguments`
- `created_at`

## Source
Lines 19–27 in `crates/scheduler/src/hitl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hitl](/crates/scheduler/src/hitl.md) |
