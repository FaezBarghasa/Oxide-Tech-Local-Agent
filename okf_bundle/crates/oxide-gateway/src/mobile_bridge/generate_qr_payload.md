---
okf_version: "0.2"
type: Function
title: generate_qr_payload
description: Generates the payload string for the desktop pairing QR code
resource: crates/oxide-gateway/src/mobile_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:52:42Z"
concept_id: crates/oxide-gateway/src/mobile_bridge/generate_qr_payload
language: rust
---

# generate_qr_payload

Generates the payload string for the desktop pairing QR code

## Signature

```rust
impl MobileBridgeManager { pub fn generate_qr_payload(&self, host_ip: &str, port: u16, host_pk_hex: &str) -> String }
```

## Visibility

- `pub`

## Docstring

Generates the payload string for the desktop pairing QR code

## Source
Lines 70–77 in `crates/oxide-gateway/src/mobile_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile_bridge](/crates/oxide-gateway/src/mobile_bridge.md) |
