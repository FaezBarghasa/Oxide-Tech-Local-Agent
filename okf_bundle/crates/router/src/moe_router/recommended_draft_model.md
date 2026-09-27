---
okf_version: "0.2"
type: Function
title: recommended_draft_model
description: Return the recommended lightweight draft model for speculative decoding pairs
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/recommended_draft_model
language: rust
---

# recommended_draft_model

Return the recommended lightweight draft model for speculative decoding pairs

## Signature

```rust
impl ExpertModel { pub fn recommended_draft_model(&self) -> Option<ExpertModel> }
```

## Visibility

- `pub`

## Docstring

Return the recommended lightweight draft model for speculative decoding pairs

## Source
Lines 68–77 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
