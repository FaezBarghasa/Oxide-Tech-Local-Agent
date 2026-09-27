---
okf_version: "0.2"
type: Class
title: CrossDomainContextSlice
description: "Multi-domain context slice containing unified Firmware, EDA, and CAD representations"
resource: crates/oxide-state/src/cross_domain_packer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/cross_domain_packer/CrossDomainContextSlice
language: rust
---

# CrossDomainContextSlice

Multi-domain context slice containing unified Firmware, EDA, and CAD representations

## Signature

```rust
pub struct CrossDomainContextSlice
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Multi-domain context slice containing unified Firmware, EDA, and CAD representations
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `task_description`
- `token_budget`
- `firmware_ast_summary`
- `eda_netlist_summary`
- `cad_feature_summary`
- `cross_domain_edges`

## Source
Lines 5–12 in `crates/oxide-state/src/cross_domain_packer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cross_domain_packer](/crates/oxide-state/src/cross_domain_packer.md) |
