//! # Core Types for Zero-Trust Remote Mesh Network (`crates/oxide-network/src/types.rs`)
//!
//! Provides fundamental types for nodes, mesh domains, overlay addresses, and wire discriminators.

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    str::FromStr,
};
use uuid::Uuid;

/// Unique identifier for a node in the mesh
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for NodeId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Self)
    }
}

impl From<Uuid> for NodeId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<NodeId> for Uuid {
    fn from(id: NodeId) -> Self {
        id.0
    }
}

/// Mesh network domain identifier (1-63 alphanumeric/hyphen characters)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MeshName(String);

impl MeshName {
    pub fn new(name: impl Into<String>) -> Result<Self, String> {
        let name = name.into();
        if name.is_empty() || name.len() > 63 {
            return Err("Mesh name must be 1-63 characters".into());
        }
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err("Mesh name must contain only alphanumeric, hyphen, or underscore".into());
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MeshName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for MeshName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Overlay IP address assignment (IPv4 in 100.64.0.0/10 CGNAT range or IPv6 ULA fd00::/8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverlayIp {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl OverlayIp {
    pub fn is_v4(&self) -> bool {
        matches!(self, Self::V4(_))
    }

    pub fn is_v6(&self) -> bool {
        matches!(self, Self::V6(_))
    }

    pub fn as_ip_addr(&self) -> IpAddr {
        match self {
            Self::V4(addr) => IpAddr::V4(*addr),
            Self::V6(addr) => IpAddr::V6(*addr),
        }
    }

    pub fn default_v4() -> Self {
        Self::V4(Ipv4Addr::new(100, 64, 0, 1))
    }

    pub fn default_v6() -> Self {
        Self::V6(Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 1))
    }
}

impl fmt::Display for OverlayIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V4(addr) => write!(f, "{}", addr),
            Self::V6(addr) => write!(f, "{}", addr),
        }
    }
}

impl FromStr for OverlayIp {
    type Err = std::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let addr: IpAddr = s.parse()?;
        Ok(match addr {
            IpAddr::V4(v4) => Self::V4(v4),
            IpAddr::V6(v6) => Self::V6(v6),
        })
    }
}

impl From<Ipv4Addr> for OverlayIp {
    fn from(addr: Ipv4Addr) -> Self {
        Self::V4(addr)
    }
}

impl From<Ipv6Addr> for OverlayIp {
    fn from(addr: Ipv6Addr) -> Self {
        Self::V6(addr)
    }
}

impl From<IpAddr> for OverlayIp {
    fn from(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(v4) => Self::V4(v4),
            IpAddr::V6(v6) => Self::V6(v6),
        }
    }
}

/// Overlay CIDR prefix for routing and subnet matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OverlayPrefix {
    pub addr: OverlayIp,
    pub prefix_len: u8,
}

impl OverlayPrefix {
    pub fn new(addr: OverlayIp, prefix_len: u8) -> Result<Self, String> {
        let max_len = match addr {
            OverlayIp::V4(_) => 32,
            OverlayIp::V6(_) => 128,
        };
        if prefix_len > max_len {
            return Err(format!(
                "Prefix length {} exceeds maximum {}",
                prefix_len, max_len
            ));
        }
        Ok(Self { addr, prefix_len })
    }

    pub fn contains(&self, ip: OverlayIp) -> bool {
        match (self.addr, ip) {
            (OverlayIp::V4(base), OverlayIp::V4(target)) => {
                if self.prefix_len == 0 {
                    return true;
                }
                let mask = !0u32 << (32 - self.prefix_len);
                u32::from(base) & mask == u32::from(target) & mask
            }
            (OverlayIp::V6(base), OverlayIp::V6(target)) => {
                if self.prefix_len == 0 {
                    return true;
                }
                let base_bytes = base.octets();
                let target_bytes = target.octets();
                let full_bytes = (self.prefix_len / 8) as usize;
                let rem_bits = self.prefix_len % 8;

                if base_bytes[..full_bytes] != target_bytes[..full_bytes] {
                    return false;
                }

                if rem_bits > 0 {
                    let mask = 0xFFu8 << (8 - rem_bits);
                    (base_bytes[full_bytes] & mask) == (target_bytes[full_bytes] & mask)
                } else {
                    true
                }
            }
            _ => false,
        }
    }
}

impl fmt::Display for OverlayPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.prefix_len)
    }
}

impl FromStr for OverlayPrefix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid CIDR format (expected IP/prefix)".into());
        }
        let addr = parts[0]
            .parse()
            .map_err(|e: std::net::AddrParseError| e.to_string())?;
        let prefix_len = parts[1]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        Self::new(addr, prefix_len)
    }
}

/// Protocol version for wire compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProtocolVersion(pub u16);

impl ProtocolVersion {
    pub const CURRENT: Self = Self(1);
    pub const MIN_COMPATIBLE: Self = Self(1);
}

/// Supported transport protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportProtocol {
    QuicDatagram,
    QuicStream,
    Udp,
    Tcp,
}

/// Node capabilities advertised in the mesh
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeCapabilities {
    pub subnet_router: bool,
    pub exit_node: bool,
    pub dns_server: bool,
    pub file_transfer: bool,
    pub ssh_server: bool,
    pub http_proxy: bool,
    pub relay: bool,
}

impl Default for NodeCapabilities {
    fn default() -> Self {
        Self {
            subnet_router: false,
            exit_node: false,
            dns_server: false,
            file_transfer: true,
            ssh_server: false,
            http_proxy: false,
            relay: false,
        }
    }
}

/// Network endpoint (IP + port)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint {
    pub addr: IpAddr,
    pub port: u16,
}

impl Endpoint {
    pub fn new(addr: IpAddr, port: u16) -> Self {
        Self { addr, port }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.addr, self.port)
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.addr, self.port)
    }
}

impl FromStr for Endpoint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let socket_addr = s.parse::<SocketAddr>().map_err(|e| e.to_string())?;
        Ok(Self::new(socket_addr.ip(), socket_addr.port()))
    }
}

/// Packet discriminator for wire protocol multiplexing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum PacketType {
    Ipv4 = 0x01,
    Ipv6 = 0x02,
    ControlKeepalive = 0x10,
    PathDiscovery = 0x11,
    RekeyNotice = 0x12,
    AclUpdate = 0x13,
    DnsQuery = 0x20,
    DnsResponse = 0x21,
    FileTransfer = 0x30,
    SshData = 0x31,
    HttpProxy = 0x32,
}

impl TryFrom<u8> for PacketType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Ipv4),
            0x02 => Ok(Self::Ipv6),
            0x10 => Ok(Self::ControlKeepalive),
            0x11 => Ok(Self::PathDiscovery),
            0x12 => Ok(Self::RekeyNotice),
            0x13 => Ok(Self::AclUpdate),
            0x20 => Ok(Self::DnsQuery),
            0x21 => Ok(Self::DnsResponse),
            0x30 => Ok(Self::FileTransfer),
            0x31 => Ok(Self::SshData),
            0x32 => Ok(Self::HttpProxy),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_uniqueness_and_serialization() {
        let n1 = NodeId::new();
        let n2 = NodeId::new();
        assert_ne!(n1, n2);

        let s = n1.to_string();
        let parsed: NodeId = s.parse().unwrap();
        assert_eq!(n1, parsed);
    }

    #[test]
    fn test_overlay_prefix_contains() {
        let prefix: OverlayPrefix = "100.64.0.0/16".parse().unwrap();
        let ip_in: OverlayIp = "100.64.12.34".parse().unwrap();
        let ip_out: OverlayIp = "100.65.1.1".parse().unwrap();

        assert!(prefix.contains(ip_in));
        assert!(!prefix.contains(ip_out));
    }
}
