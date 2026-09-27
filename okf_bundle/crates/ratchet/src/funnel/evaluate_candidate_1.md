---
okf_version: "0.2"
type: Function
title: evaluate_candidate
description: Run full three-phase funnel for candidate
resource: crates/ratchet/src/funnel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:ratchet"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/ratchet/src/funnel/evaluate_candidate_1
language: rust
---

# evaluate_candidate

Run full three-phase funnel for candidate

## Signature

```rust
pub fn evaluate_candidate(
        &self,
        surface_path: &Path,
        baseline: Option<&MetricTuple>,
        harness_digest: &str,
    ) -> (Verdict, Option<MetricTuple>)
```

## Visibility

- `pub`

## Docstring

Run full three-phase funnel for candidate

## Source
Lines 87–121 in `crates/ratchet/src/funnel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [funnel](/crates/ratchet/src/funnel.md) |
