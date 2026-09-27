---
okf_version: "0.2"
type: Function
title: run_skidl_generate
resource: crates/api/src/routes/skidl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/skidl/run_skidl_generate
language: rust
---

# run_skidl_generate

## Signature

```rust
pub fn run_skidl_generate(req: SkidlRequest) -> Result<serde_json::Value, String>
```

## Visibility

- `pub`

## Source
Lines 10–27 in `crates/api/src/routes/skidl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skidl](/crates/api/src/routes/skidl.md) |
| called_by | [handle_skidl_generate](/crates/api/src/routes/skidl/handle_skidl_generate.md) |
