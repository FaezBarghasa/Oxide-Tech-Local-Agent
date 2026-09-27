---
okf_version: "0.2"
type: Function
title: on_accept
resource: crates/router/src/feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/feedback/on_accept
language: rust
---

# on_accept

## Signature

```rust
impl UserFeedbackTracker { pub fn on_accept(sample_id: &str) -> Result<(), anyhow::Error> }
```

## Visibility

- `pub`

## Source
Lines 37–40 in `crates/router/src/feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [feedback](/crates/router/src/feedback.md) |
