---
okf_version: "0.2"
type: Function
title: json
resource: crates/web-forge/src/network_interceptor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:06Z"
concept_id: crates/web-forge/src/network_interceptor/json_1
language: rust
---

# json

## Signature

```rust
pub fn json(status_code: u16, value: &T) -> Result<Self, serde_json::Error>
```

## Type Parameters

- `T: Serialize`

## Visibility

- `pub`

## Source
Lines 68–78 in `crates/web-forge/src/network_interceptor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [network_interceptor](/crates/web-forge/src/network_interceptor.md) |
