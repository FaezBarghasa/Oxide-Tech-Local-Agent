---
okf_version: "0.2"
type: Function
title: pack_context
description: Pack multi-domain topology within the allotted token budget
resource: crates/oxide-state/src/cross_domain_packer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/cross_domain_packer/pack_context
language: rust
---

# pack_context

Pack multi-domain topology within the allotted token budget

## Signature

```rust
impl CrossDomainContextPacker { pub fn pack_context(
        task_description: &str,
        firmware_ast: &str,
        eda_netlist: &str,
        cad_features: &str,
        token_budget: usize,
    ) -> CrossDomainContextSlice }
```

## Visibility

- `pub`

## Docstring

Pack multi-domain topology within the allotted token budget

## Source
Lines 25–78 in `crates/oxide-state/src/cross_domain_packer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cross_domain_packer](/crates/oxide-state/src/cross_domain_packer.md) |
