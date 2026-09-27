---
okf_version: "0.2"
type: Function
title: lift
resource: crates/forge-rust/src/lifter/python_lifter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/lifter/python_lifter/lift
language: rust
---

# lift

## Signature

```rust
impl PythonLifter { fn lift(&self, source: &str, module_name: &str) -> Result<UirModule, LifterError> }
```

## Source
Lines 15–111 in `crates/forge-rust/src/lifter/python_lifter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [python_lifter](/crates/forge-rust/src/lifter/python_lifter.md) |
| calls | [map_py_type](/crates/forge-rust/src/lifter/python_lifter/map_py_type.md) |
