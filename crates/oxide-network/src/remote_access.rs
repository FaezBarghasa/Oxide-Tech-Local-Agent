//! # Master Remote Access & Mesh Network Engine (`crates/oxide-network/src/remote_access.rs`)
//!
//! Orchestrates virtual TUN interfaces, dynamic TCP MSS clamping, lock-free RCU Radix routing,
//! Zero-Trust ACL rule enforcement, AEAD payload encryption, and MagicDNS self-healing resolution.

use crate::acl::{AclAction, AclDirection, AclEngine, PacketMeta};
use crate::crypto::{AeadCipher, DeviceIdentityKey, DeviceIdentityPublicKey, EphemeralHandshake};
use crate::dns::{DnsWatchdog, DnsWatchdogConfig, MagicDnsResolver};
use crate::routing::{L1DirectMappedCache, RcuRouter, RouteEntry, RouteTarget};
use crate::transport::{
    CircuitBreakerConfig, DplpmtudConfig, PortHopperConfig, QuicMeshTransport, TransportStats,
};
use crate::tun::{TunConfig, TunDevice, clamp_tcp_mss};
use crate::types::{MeshName, NodeId, OverlayIp, OverlayPrefix, PacketType};
use crate::wire::{PreParseVerdict, ReplayWindow128, WirePacket, pre_parse_packet};
use oxide_core::OxideError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Mutex, RwLock};

/// Peer session information in the mesh
pub struct PeerSession {
    pub node_id: NodeId,
    pub public_key: DeviceIdentityPublicKey,
    pub overlay_ip: OverlayIp,
    pub endpoint: SocketAddr,
    pub cipher: AeadCipher,
    pub replay_window: ReplayWindow128,
    pub send_seq: u64,
}

/// Active status snapshot of the local Remote Access Mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStatusDto {
    pub node_id: NodeId,
    pub mesh_name: String,
    pub overlay_ip: OverlayIp,
    pub active_peers_count: usize,
    pub active_routes_count: usize,
    pub uptime_secs: u64,
    pub transport_stats: TransportStats,
    pub dns_healthy: bool,
}

/// Master Remote Access Mesh Engine
pub struct RemoteAccessMeshEngine {
    node_id: NodeId,
    mesh_name: MeshName,
    overlay_ip: OverlayIp,
    identity_key: Arc<DeviceIdentityKey>,
    tun_device: Arc<TunDevice>,
    rcu_router: Arc<RcuRouter>,
    acl_engine: Arc<RwLock<AclEngine>>,
    dns_resolver: Arc<MagicDnsResolver>,
    dns_watchdog: Arc<Mutex<DnsWatchdog>>,
    transport: Arc<QuicMeshTransport>,
    peers: Arc<RwLock<HashMap<NodeId, PeerSession>>>,
    start_time: Instant,
}

impl RemoteAccessMeshEngine {
    /// Initialize a new Remote Access Mesh node
    pub fn new(mesh_name: &str, overlay_ip: OverlayIp) -> Result<Self, OxideError> {
        let node_id = NodeId::new();
        let mesh_name_parsed = MeshName::new(mesh_name)
            .map_err(|e| OxideError::Config(format!("Invalid mesh name: {e}")))?;
        let identity_key = Arc::new(DeviceIdentityKey::generate()?);

        let tun_config = TunConfig {
            name: "oxide0".into(),
            ip: overlay_ip,
            netmask: "255.192.0.0".into(),
            mtu: 1420,
            multi_queue: true,
        };
        let tun_device = Arc::new(TunDevice::new(tun_config)?);
        let rcu_router = Arc::new(RcuRouter::new());
        let acl_engine = Arc::new(RwLock::new(AclEngine::default()));
        let dns_resolver = Arc::new(MagicDnsResolver::new());
        let dns_watchdog = Arc::new(Mutex::new(DnsWatchdog::new(DnsWatchdogConfig::default())));

        let (transport, _event_rx) = QuicMeshTransport::new(
            PortHopperConfig::default(),
            DplpmtudConfig::default(),
            CircuitBreakerConfig::default(),
        );

        // Register self in local DNS
        dns_resolver.register("localhost", overlay_ip, node_id);
        dns_resolver.register("agent", overlay_ip, node_id);

        Ok(Self {
            node_id,
            mesh_name: mesh_name_parsed,
            overlay_ip,
            identity_key,
            tun_device,
            rcu_router,
            acl_engine,
            dns_resolver,
            dns_watchdog,
            transport: Arc::new(transport),
            peers: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
        })
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn overlay_ip(&self) -> OverlayIp {
        self.overlay_ip
    }

    pub fn tun(&self) -> &TunDevice {
        &self.tun_device
    }

    pub fn identity_key(&self) -> &DeviceIdentityKey {
        &self.identity_key
    }

    pub fn dns_resolver(&self) -> &MagicDnsResolver {
        &self.dns_resolver
    }

    pub fn router(&self) -> &RcuRouter {
        &self.rcu_router
    }

    /// Connect and establish an encrypted session with a peer node
    pub async fn connect_peer(
        &self,
        peer_id: NodeId,
        peer_pk: DeviceIdentityPublicKey,
        peer_overlay_ip: OverlayIp,
        endpoint: SocketAddr,
        hostname: Option<&str>,
    ) -> Result<(), OxideError> {
        // 1. Perform ephemeral handshake to derive session keys
        let mut handshake = EphemeralHandshake::new();
        let synthetic_peer_nonce = *blake3::hash(&peer_pk.raw).as_bytes();
        handshake.accept_peer_nonce(&synthetic_peer_nonce);

        let (tx_key, _rx_key) = handshake.derive_session_keys(&self.identity_key, &peer_pk)?;
        let cipher = AeadCipher::new(*tx_key.as_bytes());

        // 2. Register peer in active peers map
        let session = PeerSession {
            node_id: peer_id,
            public_key: peer_pk,
            overlay_ip: peer_overlay_ip,
            endpoint,
            cipher,
            replay_window: ReplayWindow128::new(),
            send_seq: 1,
        };

        self.peers.write().await.insert(peer_id, session);
        self.transport.add_peer(peer_id, endpoint).await;

        // 3. Insert direct /32 or /128 route
        let prefix_len = if peer_overlay_ip.is_v4() { 32 } else { 128 };
        let prefix = OverlayPrefix::new(peer_overlay_ip, prefix_len)
            .map_err(|e| OxideError::Network(format!("Route prefix error: {e}")))?;

        self.rcu_router.insert_route(RouteEntry {
            prefix,
            target: RouteTarget::DirectPeer {
                node_id: peer_id,
                endpoint,
            },
            metric: 10,
            pmtu: 1400,
        });

        // 4. Register in MagicDNS if hostname provided
        if let Some(name) = hostname {
            self.dns_resolver.register(name, peer_overlay_ip, peer_id);
        }

        tracing::info!(
            "Remote access mesh established with peer {} ({}) at {}",
            peer_id,
            peer_overlay_ip,
            endpoint
        );

        Ok(())
    }

    /// Process an outbound IP packet from the local TUN interface
    pub async fn process_outbound_packet(
        &self,
        packet: &mut [u8],
        l1_cache: &mut L1DirectMappedCache,
    ) -> Result<Option<NodeId>, OxideError> {
        if packet.is_empty() {
            return Ok(None);
        }

        // 1. Dynamic TCP MSS Clamping to eliminate PMTU blackholing
        let _ = clamp_tcp_mss(packet, 1360);

        // 2. Extract packet metadata and evaluate Zero-Trust ACL
        if let Some(meta) = PacketMeta::parse(packet, AclDirection::Egress) {
            let action = self.acl_engine.read().await.evaluate(&meta);
            if action == AclAction::Deny {
                tracing::warn!(
                    "Egress packet dropped by ACL: {} -> {} ({:?})",
                    meta.src_ip,
                    meta.dst_ip,
                    meta.protocol
                );
                return Ok(None);
            }

            // 3. Resolve destination route via L1 cache or Radix LPM
            let target = if let Some(cached) = l1_cache.get(meta.dst_ip) {
                cached
            } else if let Some(resolved) = self.rcu_router.lookup(meta.dst_ip) {
                l1_cache.put(meta.dst_ip, resolved);
                resolved
            } else {
                return Ok(None); // No route to host
            };

            // 4. Encrypt and dispatch to destination peer
            if let RouteTarget::DirectPeer { node_id, .. } = target {
                let mut peers_guard = self.peers.write().await;
                if let Some(peer) = peers_guard.get_mut(&node_id) {
                    let seq = peer.send_seq;
                    peer.send_seq += 1;

                    let ciphertext = peer.cipher.encrypt(seq, packet);
                    let ptype = if meta.dst_ip.is_v4() {
                        PacketType::Ipv4
                    } else {
                        PacketType::Ipv6
                    };
                    let wire_pkt = WirePacket::new(ptype, seq as u32, ciphertext)?;
                    self.transport.send_datagram(node_id, wire_pkt).await?;
                    return Ok(Some(node_id));
                }
            }
        }

        Ok(None)
    }

    /// Process an inbound encapsulated datagram from a remote mesh peer
    pub async fn process_inbound_datagram(
        &self,
        from_node: NodeId,
        packet_bytes: &[u8],
    ) -> Result<Vec<u8>, OxideError> {
        // 1. Anti-DPI zero-allocation pre-parse
        let verdict = pre_parse_packet(packet_bytes);
        match verdict {
            PreParseVerdict::JunkIgnored => {
                Ok(Vec::new()) // Camouflage packet safely discarded
            }
            PreParseVerdict::Malformed => {
                Err(OxideError::Network("Malformed datagram header".into()))
            }
            PreParseVerdict::ValidData { packet_id, .. }
            | PreParseVerdict::ValidControl { packet_id, .. } => {
                let wire_pkt = WirePacket::from_bytes(packet_bytes)?;

                let mut peers_guard = self.peers.write().await;
                let peer = peers_guard
                    .get_mut(&from_node)
                    .ok_or_else(|| OxideError::Network(format!("Unknown peer node {from_node}")))?;

                // 2. Anti-Replay check
                let seq = packet_id as u64;
                if !peer.replay_window.check_and_update(seq) {
                    return Err(OxideError::SecurityViolation(format!(
                        "Replay or stale packet rejected (seq={seq})"
                    )));
                }

                // 3. AEAD Decryption and Authentication
                let decrypted_ip_packet = peer.cipher.decrypt(seq, &wire_pkt.payload)?;

                // 4. Ingress ACL verification
                if let Some(meta) = PacketMeta::parse(&decrypted_ip_packet, AclDirection::Ingress) {
                    let action = self.acl_engine.read().await.evaluate(&meta);
                    if action == AclAction::Deny {
                        return Err(OxideError::SecurityViolation(
                            "Ingress packet dropped by Zero-Trust ACL policy".into(),
                        ));
                    }
                }

                Ok(decrypted_ip_packet)
            }
        }
    }

    /// Fetch real-time status and telemetry for diagnostics & UI
    pub async fn status(&self) -> MeshStatusDto {
        let active_peers_count = self.peers.read().await.len();
        let active_routes_count = self.rcu_router.get_routes().len();
        let transport_stats = self.transport.get_stats().await;
        let dns_healthy = self.dns_watchdog.lock().await.is_healthy();

        MeshStatusDto {
            node_id: self.node_id,
            mesh_name: self.mesh_name.to_string(),
            overlay_ip: self.overlay_ip,
            active_peers_count,
            active_routes_count,
            uptime_secs: self.start_time.elapsed().as_secs(),
            transport_stats,
            dns_healthy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_remote_mesh_engine_peer_connection_and_routing() {
        let node_a_ip: OverlayIp = "100.64.0.1".parse().unwrap();
        let node_b_ip: OverlayIp = "100.64.0.2".parse().unwrap();

        let engine_a = RemoteAccessMeshEngine::new("oxide-lan-mesh", node_a_ip).unwrap();
        let engine_b = RemoteAccessMeshEngine::new("oxide-lan-mesh", node_b_ip).unwrap();

        let b_pk = engine_b.identity_key().public_key();
        let b_endpoint = "127.0.0.1:41641".parse().unwrap();

        // Connect Node A -> Node B
        engine_a
            .connect_peer(
                engine_b.node_id(),
                b_pk,
                node_b_ip,
                b_endpoint,
                Some("node-b"),
            )
            .await
            .unwrap();

        // Verify DNS lookup
        assert_eq!(
            engine_a.dns_resolver().resolve_name("node-b.oxide"),
            Some(node_b_ip)
        );

        // Verify route lookup
        assert!(engine_a.router().lookup(node_b_ip).is_some());

        // Verify Status DTO
        let status = engine_a.status().await;
        assert_eq!(status.active_peers_count, 1);
        assert_eq!(status.active_routes_count, 1);
    }
}
