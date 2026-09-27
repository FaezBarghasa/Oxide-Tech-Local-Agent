---
okf_version: "0.2"
type: Class
title: EvidenceBundle
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
concept_id: crates/verifier/src/evidence/EvidenceBundle
language: rust
---

# EvidenceBundle

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct EvidenceBundle
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `bundle_id`
- `task_id`
- `timestamp`
- `git_diff`
- `verifier_reports`
- `hitl_decision`
- `verified_success`

## Source
Lines 16–24 in `crates/verifier/src/evidence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [evidence](/crates/verifier/src/evidence.md) |
