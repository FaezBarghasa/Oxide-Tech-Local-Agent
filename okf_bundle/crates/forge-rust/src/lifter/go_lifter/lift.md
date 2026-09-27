---
okf_version: "0.2"
type: Function
title: lift
resource: crates/forge-rust/src/lifter/go_lifter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/lifter/go_lifter/lift
language: rust
---

# lift

## Signature

```rust
impl GoLifter { fn lift(&self, source: &str, module_name: &str) -> Result<UirModule, LifterError> }
```

## Source
Lines 16–211 in `crates/forge-rust/src/lifter/go_lifter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [go_lifter](/crates/forge-rust/src/lifter/go_lifter.md) |
| calls | [map_go_type](/crates/forge-rust/src/lifter/go_lifter/map_go_type.md) |
