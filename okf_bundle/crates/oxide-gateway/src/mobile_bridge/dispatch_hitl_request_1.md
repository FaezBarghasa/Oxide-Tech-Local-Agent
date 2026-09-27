---
okf_version: "0.2"
type: Function
title: dispatch_hitl_request
description: Dispatches an urgent HITL intervention to the paired mobile phone
resource: crates/oxide-gateway/src/mobile_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:52:42Z"
concept_id: crates/oxide-gateway/src/mobile_bridge/dispatch_hitl_request_1
language: rust
---

# dispatch_hitl_request

Dispatches an urgent HITL intervention to the paired mobile phone

## Signature

```rust
pub fn dispatch_hitl_request(
        &self,
        intervention: PendingApprovalPayload,
        datachannel_tx: &mpsc::Sender<Vec<u8>>,
    ) -> Result<(), String>
```

## Visibility

- `pub`

## Docstring

Dispatches an urgent HITL intervention to the paired mobile phone

## Source
Lines 106–128 in `crates/oxide-gateway/src/mobile_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile_bridge](/crates/oxide-gateway/src/mobile_bridge.md) |
