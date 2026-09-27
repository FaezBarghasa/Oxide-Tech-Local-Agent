---
okf_version: "0.2"
type: Function
title: lift
resource: crates/forge-rust/src/lifter/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/lifter/mod/lift_1
language: rust
---

# lift

## Signature

```rust
pub fn lift(
        source: &str,
        module_name: &str,
        lang: SourceLanguage,
    ) -> Result<UirModule, LifterError>
```

## Visibility

- `pub`

## Source
Lines 36–50 in `crates/forge-rust/src/lifter/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lifter](/crates/forge-rust/src/lifter/mod.md) |
