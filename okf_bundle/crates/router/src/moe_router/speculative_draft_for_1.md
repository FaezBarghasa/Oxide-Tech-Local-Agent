---
okf_version: "0.2"
type: Function
title: speculative_draft_for
description: "Return the recommended speculative draft model for the chosen primary expert, if any."
resource: crates/router/src/moe_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/moe_router/speculative_draft_for_1
language: rust
---

# speculative_draft_for

Return the recommended speculative draft model for the chosen primary expert, if any.

## Signature

```rust
pub fn speculative_draft_for(&self, expert: ExpertModel) -> Option<ExpertModel>
```

## Visibility

- `pub`

## Docstring

Return the recommended speculative draft model for the chosen primary expert, if any.

## Source
Lines 462–464 in `crates/router/src/moe_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_router](/crates/router/src/moe_router.md) |
