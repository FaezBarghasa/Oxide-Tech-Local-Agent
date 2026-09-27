---
okf_version: "0.2"
type: Function
title: record_feedback
description: Record runtime execution feedback from an expert
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/record_feedback_1
language: rust
---

# record_feedback

Record runtime execution feedback from an expert

## Signature

```rust
pub fn record_feedback(
        &self,
        expert: ExpertModel,
        latency_ms: f32,
        success: bool,
        quality_score: f32,
    )
```

## Visibility

- `pub`

## Docstring

Record runtime execution feedback from an expert

## Source
Lines 436–459 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
