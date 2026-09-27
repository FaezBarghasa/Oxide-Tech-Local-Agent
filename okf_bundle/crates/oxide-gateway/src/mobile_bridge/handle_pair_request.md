---
okf_version: "0.2"
type: Function
title: handle_pair_request
description: Validates pairing request and derives session encryption key
resource: crates/oxide-gateway/src/mobile_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:52:42Z"
concept_id: crates/oxide-gateway/src/mobile_bridge/handle_pair_request
language: rust
---

# handle_pair_request

Validates pairing request and derives session encryption key

## Signature

```rust
impl MobileBridgeManager { pub fn handle_pair_request(
        &self,
        client_pk: &str,
        biometric_signature: &[u8],
    ) -> Result<String, String> }
```

## Visibility

- `pub`

## Docstring

Validates pairing request and derives session encryption key

## Source
Lines 80–103 in `crates/oxide-gateway/src/mobile_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile_bridge](/crates/oxide-gateway/src/mobile_bridge.md) |
