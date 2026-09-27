---
okf_version: "0.2"
type: Function
title: handle_mobile_action
description: Handles incoming action approvals submitted from the mobile touch UI
resource: crates/oxide-gateway/src/mobile_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:52:42Z"
concept_id: crates/oxide-gateway/src/mobile_bridge/handle_mobile_action
language: rust
---

# handle_mobile_action

Handles incoming action approvals submitted from the mobile touch UI

## Signature

```rust
impl MobileBridgeManager { pub fn handle_mobile_action(&self, action: AgentControlAction) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Handles incoming action approvals submitted from the mobile touch UI

## Source
Lines 131–147 in `crates/oxide-gateway/src/mobile_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mobile_bridge](/crates/oxide-gateway/src/mobile_bridge.md) |
