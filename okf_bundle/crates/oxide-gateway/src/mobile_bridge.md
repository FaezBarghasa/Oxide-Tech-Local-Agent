---
okf_version: "0.2"
type: Module
title: mobile_bridge
description: "# Sovereign Mobile Remote Companion Bridge (`crates/oxide-gateway/src/mobile_bridge.rs`)"
resource: crates/oxide-gateway/src/mobile_bridge.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:52:42Z"
concept_id: crates/oxide-gateway/src/mobile_bridge
language: rust
---

# mobile_bridge

# Sovereign Mobile Remote Companion Bridge (`crates/oxide-gateway/src/mobile_bridge.rs`)

## Docstring

# Sovereign Mobile Remote Companion Bridge (`crates/oxide-gateway/src/mobile_bridge.rs`)

Provides zero-trust WebRTC/WebSocket signaling, encrypted QR-code pairing,
out-of-band HITL approvals, and bidirectional telemetry streaming without
third-party VPNs or cloud relays.

## Relationships

| Type | Target |
|------|--------|
| related | [MobileSignalMessage](/crates/oxide-gateway/src/mobile_bridge/MobileSignalMessage.md) |
| related | [AgentControlAction](/crates/oxide-gateway/src/mobile_bridge/AgentControlAction.md) |
| related | [PendingApprovalPayload](/crates/oxide-gateway/src/mobile_bridge/PendingApprovalPayload.md) |
| related | [MobileBridgeManager](/crates/oxide-gateway/src/mobile_bridge/MobileBridgeManager.md) |
| related | [new](/crates/oxide-gateway/src/mobile_bridge/new.md) |
| related | [generate_qr_payload](/crates/oxide-gateway/src/mobile_bridge/generate_qr_payload.md) |
| related | [handle_pair_request](/crates/oxide-gateway/src/mobile_bridge/handle_pair_request.md) |
| related | [dispatch_hitl_request](/crates/oxide-gateway/src/mobile_bridge/dispatch_hitl_request.md) |
| related | [handle_mobile_action](/crates/oxide-gateway/src/mobile_bridge/handle_mobile_action.md) |
| related | [get_pending_approvals](/crates/oxide-gateway/src/mobile_bridge/get_pending_approvals.md) |
| related | [new](/crates/oxide-gateway/src/mobile_bridge/new.md) |
| related | [generate_qr_payload](/crates/oxide-gateway/src/mobile_bridge/generate_qr_payload.md) |
| related | [handle_pair_request](/crates/oxide-gateway/src/mobile_bridge/handle_pair_request.md) |
| related | [dispatch_hitl_request](/crates/oxide-gateway/src/mobile_bridge/dispatch_hitl_request.md) |
| related | [handle_mobile_action](/crates/oxide-gateway/src/mobile_bridge/handle_mobile_action.md) |
| related | [get_pending_approvals](/crates/oxide-gateway/src/mobile_bridge/get_pending_approvals.md) |
| related | [test_qr_payload_generation](/crates/oxide-gateway/src/mobile_bridge/test_qr_payload_generation.md) |
| related | [test_mobile_pairing_handshake](/crates/oxide-gateway/src/mobile_bridge/test_mobile_pairing_handshake.md) |
| related | [test_hitl_dispatch_and_approval_flow](/crates/oxide-gateway/src/mobile_bridge/test_hitl_dispatch_and_approval_flow.md) |
| related | [test_emergency_halt_action](/crates/oxide-gateway/src/mobile_bridge/test_emergency_halt_action.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
