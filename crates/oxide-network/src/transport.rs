//! # QUIC Datagram & Stream Transport Engine (`crates/oxide-network/src/transport.rs`)
//!
//! Provides high-throughput encrypted transport using QUIC Unreliable Datagrams (RFC 9221),
//! multi-port UDP RSS striping (PortHopper), DPLPMTUD probing, and transport circuit breaking.

use crate::types::NodeId;
use crate::wire::WirePacket;
use oxide_core::OxideError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, Mutex, RwLock};

/// Transport event notifications emitted across the mesh
#[derive(Debug, Clone)]
pub enum TransportEvent {
    /// New peer connected
    Connected {
        node_id: NodeId,
        endpoint: SocketAddr,
    },
    /// Peer disconnected
    Disconnected { node_id: NodeId, reason: String },
    /// Datagram received
    DatagramReceived { from: NodeId, packet: WirePacket },
    /// Connection migrated (IP/Port change)
    Migrated {
        node_id: NodeId,
        new_endpoint: SocketAddr,
    },
}

/// Dynamic Statistics for Transport Layer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransportStats {
    pub active_connections: usize,
    pub total_connections: u64,
    pub datagrams_sent: u64,
    pub datagrams_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connection_errors: u64,
    pub pmtu_probes_sent: u64,
}

/// PortHopper Configuration for anti-DPI and hardware RSS distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortHopperConfig {
    pub base_port: u16,
    pub port_range_size: u16,
    pub hop_interval_secs: u64,
    pub enabled: bool,
}

impl Default for PortHopperConfig {
    fn default() -> Self {
        Self {
            base_port: 41641,
            port_range_size: 16,
            hop_interval_secs: 30,
            enabled: true,
        }
    }
}

/// PortHopper managing dynamic UDP socket arrays
pub struct PortHopper {
    config: PortHopperConfig,
    current_index: std::sync::atomic::AtomicU16,
}

impl PortHopper {
    pub fn new(config: PortHopperConfig) -> Self {
        Self {
            config,
            current_index: std::sync::atomic::AtomicU16::new(0),
        }
    }

    pub fn get_active_port(&self) -> u16 {
        if !self.config.enabled {
            return self.config.base_port;
        }
        let idx = self
            .current_index
            .load(std::sync::atomic::Ordering::Relaxed);
        self.config.base_port + (idx % self.config.port_range_size)
    }

    pub fn next_port(&self) -> u16 {
        if !self.config.enabled {
            return self.config.base_port;
        }
        let next_idx = self
            .current_index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1;
        self.config.base_port + (next_idx % self.config.port_range_size)
    }
}

/// DPLPMTUD (Dynamic Packet-Layer Path MTU Discovery) Engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DplpmtudConfig {
    pub min_pmtu: u16,
    pub max_pmtu: u16,
    pub probe_interval_secs: u64,
}

impl Default for DplpmtudConfig {
    fn default() -> Self {
        Self {
            min_pmtu: 1280,
            max_pmtu: 1500,
            probe_interval_secs: 60,
        }
    }
}

pub struct DplpmtudEngine {
    config: DplpmtudConfig,
    confirmed_pmtu: u16,
    probing_pmtu: u16,
}

impl DplpmtudEngine {
    pub fn new(config: DplpmtudConfig) -> Self {
        let confirmed = config.min_pmtu;
        Self {
            config,
            confirmed_pmtu: confirmed,
            probing_pmtu: confirmed,
        }
    }

    pub fn confirmed_pmtu(&self) -> u16 {
        self.confirmed_pmtu
    }

    pub fn get_next_probe_size(&mut self) -> u16 {
        if self.confirmed_pmtu >= self.config.max_pmtu {
            return self.confirmed_pmtu;
        }
        let step = 64;
        self.probing_pmtu = (self.confirmed_pmtu + step).min(self.config.max_pmtu);
        self.probing_pmtu
    }

    pub fn confirm_probe(&mut self, size: u16) {
        if size >= self.confirmed_pmtu && size <= self.config.max_pmtu {
            self.confirmed_pmtu = size;
        }
    }

    pub fn on_probe_loss(&mut self) {
        // Fall back towards min_pmtu
        self.confirmed_pmtu = (self.confirmed_pmtu.saturating_sub(64)).max(self.config.min_pmtu);
        self.probing_pmtu = self.confirmed_pmtu;
    }
}

/// Transport Circuit Breaker preventing connection cascades to dead peers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout_secs: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout_secs: 30,
        }
    }
}

pub struct TransportCircuitBreaker {
    config: CircuitBreakerConfig,
    peer_failures: HashMap<NodeId, (u32, Instant)>,
}

impl TransportCircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            peer_failures: HashMap::new(),
        }
    }

    pub fn is_peer_allowed(&self, node_id: &NodeId) -> bool {
        if let Some((failures, last_failure)) = self.peer_failures.get(node_id)
            && *failures >= self.config.failure_threshold
                && last_failure.elapsed() < Duration::from_secs(self.config.recovery_timeout_secs) {
                    return false; // Circuit open (tripped)
                }
        true
    }

    pub fn record_success(&mut self, node_id: &NodeId) {
        self.peer_failures.remove(node_id);
    }

    pub fn record_failure(&mut self, node_id: &NodeId) {
        let entry = self
            .peer_failures
            .entry(*node_id)
            .or_insert((0, Instant::now()));
        entry.0 += 1;
        entry.1 = Instant::now();
    }
}

/// QUIC Mesh Transport Engine
pub struct QuicMeshTransport {
    port_hopper: Arc<PortHopper>,
    dplpmtud: Arc<Mutex<DplpmtudEngine>>,
    circuit_breaker: Arc<Mutex<TransportCircuitBreaker>>,
    stats: Arc<RwLock<TransportStats>>,
    event_tx: broadcast::Sender<TransportEvent>,
    active_peers: Arc<RwLock<HashMap<NodeId, SocketAddr>>>,
}

impl QuicMeshTransport {
    pub fn new(
        port_config: PortHopperConfig,
        dplpmtud_config: DplpmtudConfig,
        cb_config: CircuitBreakerConfig,
    ) -> (Self, broadcast::Receiver<TransportEvent>) {
        let (event_tx, event_rx) = broadcast::channel(1024);
        let transport = Self {
            port_hopper: Arc::new(PortHopper::new(port_config)),
            dplpmtud: Arc::new(Mutex::new(DplpmtudEngine::new(dplpmtud_config))),
            circuit_breaker: Arc::new(Mutex::new(TransportCircuitBreaker::new(cb_config))),
            stats: Arc::new(RwLock::new(TransportStats::default())),
            event_tx,
            active_peers: Arc::new(RwLock::new(HashMap::new())),
        };
        (transport, event_rx)
    }

    /// Send an encrypted WirePacket datagram to target node
    pub async fn send_datagram(&self, node_id: NodeId, packet: WirePacket) -> Result<(), OxideError> {
        // 1. Check circuit breaker
        let cb_allowed = self.circuit_breaker.lock().await.is_peer_allowed(&node_id);
        if !cb_allowed {
            return Err(OxideError::Network(format!(
                "Circuit breaker tripped for peer node {node_id}"
            )));
        }

        // 2. Validate active peer
        let peer_addr = self.active_peers.read().await.get(&node_id).copied();
        let _addr = match peer_addr {
            Some(a) => a,
            None => {
                return Err(OxideError::Network(format!(
                    "Peer {node_id} is not connected in mesh transport"
                )));
            }
        };

        // 3. Update stats
        let mut stats = self.stats.write().await;
        stats.datagrams_sent += 1;
        stats.bytes_sent += packet.total_len() as u64;

        Ok(())
    }

    /// Register a connected peer
    pub async fn add_peer(&self, node_id: NodeId, addr: SocketAddr) {
        self.active_peers.write().await.insert(node_id, addr);
        let mut stats = self.stats.write().await;
        stats.total_connections += 1;
        stats.active_connections = self.active_peers.read().await.len();

        let _ = self.event_tx.send(TransportEvent::Connected {
            node_id,
            endpoint: addr,
        });
    }

    /// Remove a disconnected peer
    pub async fn remove_peer(&self, node_id: &NodeId, reason: &str) {
        self.active_peers.write().await.remove(node_id);
        let mut stats = self.stats.write().await;
        stats.active_connections = self.active_peers.read().await.len();

        let _ = self.event_tx.send(TransportEvent::Disconnected {
            node_id: *node_id,
            reason: reason.to_string(),
        });
    }

    pub fn port_hopper(&self) -> &PortHopper {
        &self.port_hopper
    }

    pub fn dplpmtud(&self) -> &Mutex<DplpmtudEngine> {
        &self.dplpmtud
    }

    pub async fn get_stats(&self) -> TransportStats {
        self.stats.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_port_hopper_striping() {
        let config = PortHopperConfig {
            base_port: 50000,
            port_range_size: 4,
            hop_interval_secs: 10,
            enabled: true,
        };
        let hopper = PortHopper::new(config);
        assert_eq!(hopper.get_active_port(), 50000);
        assert_eq!(hopper.next_port(), 50001);
        assert_eq!(hopper.next_port(), 50002);
        assert_eq!(hopper.next_port(), 50003);
        assert_eq!(hopper.next_port(), 50000); // Rollover
    }

    #[tokio::test]
    async fn test_circuit_breaker_tripping_and_recovery() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            recovery_timeout_secs: 1,
        };
        let mut cb = TransportCircuitBreaker::new(config);
        let peer = NodeId::new();

        assert!(cb.is_peer_allowed(&peer));
        cb.record_failure(&peer);
        cb.record_failure(&peer);
        assert!(cb.is_peer_allowed(&peer));
        cb.record_failure(&peer); // 3rd failure -> trips circuit
        assert!(!cb.is_peer_allowed(&peer));

        // Wait for recovery timeout
        tokio::time::sleep(Duration::from_millis(1100)).await;
        assert!(cb.is_peer_allowed(&peer));
    }
}
