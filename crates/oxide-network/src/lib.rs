//! # Zero-Trust Remote Access & Mesh Network Subsystem (`crates/oxide-network`)
//!
//! Master remote overlay networking stack implementing:
//! - Pure-Rust multi-queue TUN interface abstraction and dynamic TCP MSS clamping
//! - 128-bit sliding bitmask anti-replay protection and anti-DPI junk frame classification
//! - Lock-free Read-Copy-Update (RCU) Radix Longest-Prefix-Match routing with L1 cache
//! - Ed25519 identity, Blake3 KDF, and authenticated AEAD encryption/decryption
//! - Quinn QUIC datagram (RFC 9221) transport with PortHopper RSS striping & DPLPMTUD
//! - Zero-Trust Access Control Lists (ACL) microsegmentation
//! - MagicDNS internal `.oxide` resolution with OS resolver self-healing watchdog
//! - Seamless LAN mDNS discovery and mobile companion gateway bridging.

pub mod acl;
pub mod crypto;
pub mod dns;
pub mod remote_access;
pub mod routing;
pub mod transport;
pub mod tun;
pub mod types;
pub mod wire;

pub use acl::{AclAction, AclDirection, AclEngine, AclRule, PacketMeta, Protocol};
pub use crypto::{
    AeadCipher, DeviceIdentityKey, DeviceIdentityPublicKey, DeviceSignature, EphemeralHandshake,
    KeyFingerprint, SessionKey,
};
pub use dns::{DnsRecord, DnsWatchdog, DnsWatchdogConfig, MagicDnsResolver};
pub use remote_access::{MeshStatusDto, PeerSession, RemoteAccessMeshEngine};
pub use routing::{L1DirectMappedCache, RadixRoutingTable, RcuRouter, RouteEntry, RouteTarget};
pub use transport::{
    CircuitBreakerConfig, DplpmtudConfig, DplpmtudEngine, PortHopper, PortHopperConfig,
    QuicMeshTransport, TransportCircuitBreaker, TransportEvent, TransportStats,
};
pub use tun::{clamp_tcp_mss, TunConfig, TunDevice};
pub use types::{
    Endpoint, MeshName, NodeCapabilities, NodeId, OverlayIp, OverlayPrefix, PacketType,
    ProtocolVersion, TransportProtocol,
};
pub use wire::{
    pre_parse_packet, AclRuleWire, AclUpdatePayload, BatchPacket, KeepalivePayload,
    PacketHeader, PathDiscoveryPayload, PreParseVerdict, RekeyNoticePayload, ReplayWindow128,
    WirePacket, DISCRIMINATOR_OXIDE_CTRL, DISCRIMINATOR_OXIDE_DATA, JUNK_PREAMBLE_MAGIC_1,
    JUNK_PREAMBLE_MAGIC_2, MAX_PACKET_SIZE, MIN_PACKET_SIZE, PROTOCOL_MAGIC,
};

use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;

/// Local Area Network mDNS broadcast service
pub struct LanDiscovery {
    mdns: ServiceDaemon,
}

impl LanDiscovery {
    pub fn new() -> Result<Self, mdns_sd::Error> {
        let mdns = ServiceDaemon::new()?;
        Ok(Self { mdns })
    }

    pub fn broadcast_service(&self, port: u16) -> Result<(), mdns_sd::Error> {
        let service_type = "_oxide-agent._tcp.local.";
        let instance_name = "oxide_tech_os";
        let host_name = "oxide.local.";
        let ip = "127.0.0.1";
        let properties: HashMap<String, String> =
            [("version".to_string(), "0.5.0".to_string())].into();

        let service_info =
            ServiceInfo::new(service_type, instance_name, host_name, ip, port, properties)?;

        self.mdns.register(service_info)?;
        tracing::info!(
            "mDNS broadcast active for _oxide-agent._tcp.local on port {}",
            port
        );
        Ok(())
    }
}
