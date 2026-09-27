---
okf_version: "0.2"
type: Function
title: export_to_directory
description: Exports the bundle to a structured directory
resource: crates/verifier/src/evidence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/evidence/export_to_directory
language: rust
---

# export_to_directory

Exports the bundle to a structured directory

## Signature

```rust
impl EvidenceBundle { pub fn export_to_directory(&self, output_dir: impl AsRef<Path>) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Exports the bundle to a structured directory

## Source
Lines 47–69 in `crates/verifier/src/evidence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [evidence](/crates/verifier/src/evidence.md) |
