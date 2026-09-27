---
okf_version: "0.2"
type: Function
title: get_pending_approvals
description: List active pending approvals awaiting mobile operator response
resource: crates/oxide-gateway/src/mobile_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:52:42Z"
concept_id: crates/oxide-gateway/src/mobile_bridge/get_pending_approvals_1
language: rust
---

# get_pending_approvals

List active pending approvals awaiting mobile operator response

## Signature

```rust
pub fn get_pending_approvals(&self) -> Vec<PendingApprovalPayload>
```

## Visibility

- `pub`

## Docstring

List active pending approvals awaiting mobile operator response

## Source
Lines 150–152 in `crates/oxide-gateway/src/mobile_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile_bridge](/crates/oxide-gateway/src/mobile_bridge.md) |
