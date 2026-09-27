---
okf_version: "0.2"
type: Module
title: funnel
description: "# Three-Phase Candidate Funnel"
resource: crates/ratchet/src/funnel.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:ratchet"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/ratchet/src/funnel
language: rust
---

# funnel

# Three-Phase Candidate Funnel

## Docstring

# Three-Phase Candidate Funnel

Phase 1 Validate (CPU, ~1.5s) -> Phase 2 Probe (1 GPU step, ~2s) -> Phase 3 Budget (5 min)

## Relationships

| Type | Target |
|------|--------|
| related | [CandidateFunnel](/crates/ratchet/src/funnel/CandidateFunnel.md) |
| related | [default](/crates/ratchet/src/funnel/default.md) |
| related | [default](/crates/ratchet/src/funnel/default.md) |
| related | [validate_phase1](/crates/ratchet/src/funnel/validate_phase1.md) |
| related | [evaluate_candidate](/crates/ratchet/src/funnel/evaluate_candidate.md) |
| related | [validate_phase1](/crates/ratchet/src/funnel/validate_phase1.md) |
| related | [evaluate_candidate](/crates/ratchet/src/funnel/evaluate_candidate.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
