---
okf_version: "0.2"
type: Class
title: CrossDomainVerificationReport
description: Verification outcome report for a multi-physics cycle
resource: crates/cross-domain-verifier/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:cross-domain-verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:48:30Z"
concept_id: crates/cross-domain-verifier/src/lib/CrossDomainVerificationReport
language: rust
---

# CrossDomainVerificationReport

Verification outcome report for a multi-physics cycle

## Signature

```rust
pub struct CrossDomainVerificationReport
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Verification outcome report for a multi-physics cycle
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `dtx_id`
- `passed`
- `firmware_power_watts`
- `peak_temperature_c`
- `clearance_margin_mm`
- `thermal_fea`
- `iterations`
- `residual`
- `recommended_action`

## Source
Lines 31–41 in `crates/cross-domain-verifier/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/cross-domain-verifier/src/lib.md) |
