---
okf_version: "0.2"
type: Function
title: handle_skidl_generate
description: "[post(\"/api/skidl/generate\")]"
resource: crates/api/src/routes/skidl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/skidl/handle_skidl_generate
language: rust
---

# handle_skidl_generate

[post("/api/skidl/generate")]

## Signature

```rust
pub fn handle_skidl_generate(req: web::Json<SkidlRequest>) -> impl Responder
```

## Decorators

- `post("/api/skidl/generate")`

## Visibility

- `pub`

## Docstring

[post("/api/skidl/generate")]

## Source
Lines 30–38 in `crates/api/src/routes/skidl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [skidl](/crates/api/src/routes/skidl.md) |
| calls | [run_skidl_generate](/crates/api/src/routes/skidl/run_skidl_generate.md) |
