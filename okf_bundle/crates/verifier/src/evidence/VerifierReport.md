---
okf_version: "0.2"
type: Class
title: VerifierReport
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/verifier/src/evidence.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/evidence/VerifierReport
language: rust
---

# VerifierReport

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct VerifierReport
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `stage`
- `passed`
- `stdout`
- `stderr`
- `duration_ms`

## Source
Lines 7–13 in `crates/verifier/src/evidence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [evidence](/crates/verifier/src/evidence.md) |
