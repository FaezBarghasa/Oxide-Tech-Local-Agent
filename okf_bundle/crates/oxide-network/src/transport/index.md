# transport

## Classs

- [CircuitBreakerConfig](CircuitBreakerConfig.md) — Transport Circuit Breaker preventing connection cascades to dead peers
- [DplpmtudConfig](DplpmtudConfig.md) — DPLPMTUD (Dynamic Packet-Layer Path MTU Discovery) Engine
- [DplpmtudEngine](DplpmtudEngine.md)
- [PortHopper](PortHopper.md) — PortHopper managing dynamic UDP socket arrays
- [PortHopperConfig](PortHopperConfig.md) — PortHopper Configuration for anti-DPI and hardware RSS distribution
- [QuicMeshTransport](QuicMeshTransport.md) — QUIC Mesh Transport Engine
- [TransportCircuitBreaker](TransportCircuitBreaker.md)
- [TransportEvent](TransportEvent.md) — Transport event notifications emitted across the mesh
- [TransportStats](TransportStats.md) — Dynamic Statistics for Transport Layer

## Functions

- [add_peer](add_peer.md) — Register a connected peer
- [add_peer](add_peer_1.md) — Register a connected peer
- [confirm_probe](confirm_probe.md)
- [confirm_probe](confirm_probe_1.md)
- [confirmed_pmtu](confirmed_pmtu.md)
- [confirmed_pmtu](confirmed_pmtu_1.md)
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [default](default_4.md)
- [default](default_5.md)
- [dplpmtud](dplpmtud.md)
- [dplpmtud](dplpmtud_1.md)
- [get_active_port](get_active_port.md)
- [get_active_port](get_active_port_1.md)
- [get_next_probe_size](get_next_probe_size.md)
- [get_next_probe_size](get_next_probe_size_1.md)
- [get_stats](get_stats.md)
- [get_stats](get_stats_1.md)
- [is_peer_allowed](is_peer_allowed.md)
- [is_peer_allowed](is_peer_allowed_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [new](new_6.md)
- [new](new_7.md)
- [next_port](next_port.md)
- [next_port](next_port_1.md)
- [on_probe_loss](on_probe_loss.md)
- [on_probe_loss](on_probe_loss_1.md)
- [port_hopper](port_hopper.md)
- [port_hopper](port_hopper_1.md)
- [record_failure](record_failure.md)
- [record_failure](record_failure_1.md)
- [record_success](record_success.md)
- [record_success](record_success_1.md)
- [remove_peer](remove_peer.md) — Remove a disconnected peer
- [remove_peer](remove_peer_1.md) — Remove a disconnected peer
- [send_datagram](send_datagram.md) — Send an encrypted WirePacket datagram to target node
- [send_datagram](send_datagram_1.md) — Send an encrypted WirePacket datagram to target node
- [test_circuit_breaker_tripping_and_recovery](test_circuit_breaker_tripping_and_recovery.md) — [tokio::test]
- [test_port_hopper_striping](test_port_hopper_striping.md) — [tokio::test]
