---
okf_version: "0.2"
type: Module
title: transport
description: "# QUIC Datagram & Stream Transport Engine (`crates/oxide-network/src/transport.rs`)"
resource: crates/oxide-network/src/transport.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:10:01Z"
concept_id: crates/oxide-network/src/transport
language: rust
---

# transport

# QUIC Datagram & Stream Transport Engine (`crates/oxide-network/src/transport.rs`)

## Docstring

# QUIC Datagram & Stream Transport Engine (`crates/oxide-network/src/transport.rs`)

Provides high-throughput encrypted transport using QUIC Unreliable Datagrams (RFC 9221),
multi-port UDP RSS striping (PortHopper), DPLPMTUD probing, and transport circuit breaking.

## Relationships

| Type | Target |
|------|--------|
| related | [TransportEvent](/crates/oxide-network/src/transport/TransportEvent.md) |
| related | [TransportStats](/crates/oxide-network/src/transport/TransportStats.md) |
| related | [PortHopperConfig](/crates/oxide-network/src/transport/PortHopperConfig.md) |
| related | [default](/crates/oxide-network/src/transport/default.md) |
| related | [default](/crates/oxide-network/src/transport/default.md) |
| related | [PortHopper](/crates/oxide-network/src/transport/PortHopper.md) |
| related | [new](/crates/oxide-network/src/transport/new.md) |
| related | [get_active_port](/crates/oxide-network/src/transport/get_active_port.md) |
| related | [next_port](/crates/oxide-network/src/transport/next_port.md) |
| related | [new](/crates/oxide-network/src/transport/new.md) |
| related | [get_active_port](/crates/oxide-network/src/transport/get_active_port.md) |
| related | [next_port](/crates/oxide-network/src/transport/next_port.md) |
| related | [DplpmtudConfig](/crates/oxide-network/src/transport/DplpmtudConfig.md) |
| related | [default](/crates/oxide-network/src/transport/default.md) |
| related | [default](/crates/oxide-network/src/transport/default.md) |
| related | [DplpmtudEngine](/crates/oxide-network/src/transport/DplpmtudEngine.md) |
| related | [new](/crates/oxide-network/src/transport/new.md) |
| related | [confirmed_pmtu](/crates/oxide-network/src/transport/confirmed_pmtu.md) |
| related | [get_next_probe_size](/crates/oxide-network/src/transport/get_next_probe_size.md) |
| related | [confirm_probe](/crates/oxide-network/src/transport/confirm_probe.md) |
| related | [on_probe_loss](/crates/oxide-network/src/transport/on_probe_loss.md) |
| related | [new](/crates/oxide-network/src/transport/new.md) |
| related | [confirmed_pmtu](/crates/oxide-network/src/transport/confirmed_pmtu.md) |
| related | [get_next_probe_size](/crates/oxide-network/src/transport/get_next_probe_size.md) |
| related | [confirm_probe](/crates/oxide-network/src/transport/confirm_probe.md) |
| related | [on_probe_loss](/crates/oxide-network/src/transport/on_probe_loss.md) |
| related | [CircuitBreakerConfig](/crates/oxide-network/src/transport/CircuitBreakerConfig.md) |
| related | [default](/crates/oxide-network/src/transport/default.md) |
| related | [default](/crates/oxide-network/src/transport/default.md) |
| related | [TransportCircuitBreaker](/crates/oxide-network/src/transport/TransportCircuitBreaker.md) |
| related | [new](/crates/oxide-network/src/transport/new.md) |
| related | [is_peer_allowed](/crates/oxide-network/src/transport/is_peer_allowed.md) |
| related | [record_success](/crates/oxide-network/src/transport/record_success.md) |
| related | [record_failure](/crates/oxide-network/src/transport/record_failure.md) |
| related | [new](/crates/oxide-network/src/transport/new.md) |
| related | [is_peer_allowed](/crates/oxide-network/src/transport/is_peer_allowed.md) |
| related | [record_success](/crates/oxide-network/src/transport/record_success.md) |
| related | [record_failure](/crates/oxide-network/src/transport/record_failure.md) |
| related | [QuicMeshTransport](/crates/oxide-network/src/transport/QuicMeshTransport.md) |
| related | [new](/crates/oxide-network/src/transport/new.md) |
| related | [send_datagram](/crates/oxide-network/src/transport/send_datagram.md) |
| related | [add_peer](/crates/oxide-network/src/transport/add_peer.md) |
| related | [remove_peer](/crates/oxide-network/src/transport/remove_peer.md) |
| related | [port_hopper](/crates/oxide-network/src/transport/port_hopper.md) |
| related | [dplpmtud](/crates/oxide-network/src/transport/dplpmtud.md) |
| related | [get_stats](/crates/oxide-network/src/transport/get_stats.md) |
| related | [new](/crates/oxide-network/src/transport/new.md) |
| related | [send_datagram](/crates/oxide-network/src/transport/send_datagram.md) |
| related | [add_peer](/crates/oxide-network/src/transport/add_peer.md) |
| related | [remove_peer](/crates/oxide-network/src/transport/remove_peer.md) |
| related | [port_hopper](/crates/oxide-network/src/transport/port_hopper.md) |
| related | [dplpmtud](/crates/oxide-network/src/transport/dplpmtud.md) |
| related | [get_stats](/crates/oxide-network/src/transport/get_stats.md) |
| related | [test_port_hopper_striping](/crates/oxide-network/src/transport/test_port_hopper_striping.md) |
| related | [test_circuit_breaker_tripping_and_recovery](/crates/oxide-network/src/transport/test_circuit_breaker_tripping_and_recovery.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
