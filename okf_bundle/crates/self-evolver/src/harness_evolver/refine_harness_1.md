---
okf_version: "0.2"
type: Function
title: refine_harness
description: Diagnoses an oscillation/compiler error trace and appends a prompt note rule to H.ρ
resource: crates/self-evolver/src/harness_evolver.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/self-evolver/src/harness_evolver/refine_harness_1
language: rust
---

# refine_harness

Diagnoses an oscillation/compiler error trace and appends a prompt note rule to H.ρ

## Signature

```rust
pub fn refine_harness(
        &self,
        domain: &str,
        error_fingerprint: &str,
        raw_stderr: &str,
    ) -> Result<HarnessRefinement>
```

## Visibility

- `pub`

## Docstring

Diagnoses an oscillation/compiler error trace and appends a prompt note rule to H.ρ

## Source
Lines 34–84 in `crates/self-evolver/src/harness_evolver.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness_evolver](/crates/self-evolver/src/harness_evolver.md) |
