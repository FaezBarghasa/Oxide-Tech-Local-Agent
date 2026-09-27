---
okf_version: "0.2"
type: Function
title: new
description: "Create a gate with the given threshold and a `max_errors` cap of 20."
resource: crates/oxide-gateway/src/quality.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/quality/new
language: rust
---

# new

Create a gate with the given threshold and a `max_errors` cap of 20.

## Signature

```rust
impl QualityGate { pub fn new(threshold: f32) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a gate with the given threshold and a `max_errors` cap of 20.

## Source
Lines 18–23 in `crates/oxide-gateway/src/quality.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [quality](/crates/oxide-gateway/src/quality.md) |
