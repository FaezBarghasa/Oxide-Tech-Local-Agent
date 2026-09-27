---
okf_version: "0.2"
type: Function
title: status_summary
description: Returns a human-readable summary of the current routing state for the
resource: crates/oxide-gateway/src/router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/router/status_summary_1
language: rust
---

# status_summary

Returns a human-readable summary of the current routing state for the

## Signature

```rust
pub fn status_summary(&self) -> serde_json::Value
```

## Visibility

- `pub`

## Docstring

Returns a human-readable summary of the current routing state for the
`/api/status` endpoint.

## Source
Lines 159–170 in `crates/oxide-gateway/src/router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [router](/crates/oxide-gateway/src/router.md) |
