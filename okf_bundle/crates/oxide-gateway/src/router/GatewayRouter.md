---
okf_version: "0.2"
type: Class
title: GatewayRouter
description: The smart routing controller.
resource: crates/oxide-gateway/src/router.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/router/GatewayRouter
language: rust
---

# GatewayRouter

The smart routing controller.

## Signature

```rust
pub struct GatewayRouter
```

## Visibility

- `pub`

## Docstring

The smart routing controller.

Holds all three coder clients and the thinker client.  On each generation
request it:
1. Probes the primary online endpoint for latency.
2. If too slow or unreachable, downgrades to secondary or local.
3. After a cargo-check pass, scores the output with `QualityGate`.
4. If quality is below threshold it flips to the other coder.

## Methods

- `thinker`
- `online_primary`
- `online_secondary`
- `local`
- `multi_agent`
- `quality_gate`
- `latency_threshold_ms`
- `current_backend`

## Source
Lines 52–63 in `crates/oxide-gateway/src/router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [router](/crates/oxide-gateway/src/router.md) |
