---
okf_version: "0.2"
type: Function
title: run_pcb_drc
resource: crates/circuit-forge/src/drc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/circuit-forge/src/drc/run_pcb_drc_1
language: rust
---

# run_pcb_drc

## Signature

```rust
pub fn run_pcb_drc(
        &self,
        pcb_file: &Path,
        deck: ManufacturerDeck,
    ) -> Result<DrcReport, DrcError>
```

## Visibility

- `pub`

## Source
Lines 61–161 in `crates/circuit-forge/src/drc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [drc](/crates/circuit-forge/src/drc.md) |
