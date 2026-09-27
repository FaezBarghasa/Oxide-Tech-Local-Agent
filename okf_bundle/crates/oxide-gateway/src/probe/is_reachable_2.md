---
okf_version: "0.2"
type: Function
title: is_reachable
resource: crates/oxide-gateway/src/probe.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/probe/is_reachable_2
language: rust
---

# is_reachable

## Signature

```rust
pub fn is_reachable(base_url: &str, timeout_ms: u64) -> bool
```

## Visibility

- `pub`

## Source
Lines 167–169 in `crates/oxide-gateway/src/probe.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [probe](/crates/oxide-gateway/src/probe.md) |
| calls | [get_prober](/crates/oxide-gateway/src/probe/get_prober.md) |
