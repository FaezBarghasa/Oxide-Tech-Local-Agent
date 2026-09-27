---
okf_version: "0.2"
type: Function
title: call
resource: crates/oxide-gateway/src/middleware.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-gateway/src/middleware/call
language: rust
---

# call

## Signature

```rust
impl ApiKeyAuthMiddleware<S> { fn call(&self, req: ServiceRequest) -> Self::Future }
```

## Type Parameters

- `S`
- `B`

## Source
Lines 51–78 in `crates/oxide-gateway/src/middleware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [middleware](/crates/oxide-gateway/src/middleware.md) |
