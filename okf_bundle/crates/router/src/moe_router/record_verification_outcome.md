---
okf_version: "0.2"
type: Function
title: record_verification_outcome
description: Record verification outcome directly to adjust quality EMA and circuit breaker
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/record_verification_outcome
language: rust
---

# record_verification_outcome

Record verification outcome directly to adjust quality EMA and circuit breaker

## Signature

```rust
impl AdaptiveMoeGatingRouter { pub fn record_verification_outcome(
        &self,
        expert: ExpertModel,
        passed: bool,
        duration_ms: u64,
        score: f32,
    ) }
```

## Visibility

- `pub`

## Docstring

Record verification outcome directly to adjust quality EMA and circuit breaker

## Source
Lines 467–475 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
