---
okf_version: "0.2"
type: Function
title: suggest_next_action
description: Generates structured suggestions for the next parametric operation or hierarchy delta.
resource: crates/visual-forge/src/optimizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:visual-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T20:00:45Z"
concept_id: crates/visual-forge/src/optimizer/suggest_next_action_1
language: rust
---

# suggest_next_action

Generates structured suggestions for the next parametric operation or hierarchy delta.

## Signature

```rust
pub fn suggest_next_action(&self, report: &GeometryCorrectionReport) -> String
```

## Visibility

- `pub`

## Docstring

Generates structured suggestions for the next parametric operation or hierarchy delta.

## Source
Lines 57–88 in `crates/visual-forge/src/optimizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [optimizer](/crates/visual-forge/src/optimizer.md) |
