---
okf_version: "0.2"
type: Function
title: generate_error_map
description: Generate a structured topology error map if verification fails.
resource: crates/scene-forge/src/verifier.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scene-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:47:23Z"
concept_id: crates/scene-forge/src/verifier/generate_error_map_1
language: rust
---

# generate_error_map

Generate a structured topology error map if verification fails.

## Signature

```rust
pub fn generate_error_map(&self, report: &ManifoldReport) -> Option<TopologyErrorMap>
```

## Visibility

- `pub`

## Docstring

Generate a structured topology error map if verification fails.

## Source
Lines 150–175 in `crates/scene-forge/src/verifier.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [verifier](/crates/scene-forge/src/verifier.md) |
