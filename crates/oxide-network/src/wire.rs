//! # Wire Protocol Framing for Data & Control Plane Packets (`crates/oxide-network/src/wire.rs`)
//!
//! Provides binary envelope encoding, 128-bit anti-replay protection,
//! GRO/GSO batching, and zero-allocation anti-DPI packet pre-parsing.

use crate::types::{
    Endpoint, NodeCapabilities, NodeId, OverlayPrefix, PacketType, ProtocolVersion,
};
use bytes::{BufMut, BytesMut};
use oxide_core::OxideError;
use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Maximum packet size (64KB for GRO/GSO aggregates)
pub const MAX_PACKET_SIZE: usize = 65536;
/// Minimum packet size (16-byte header only)
pub const MIN_PACKET_SIZE: usize = 16;
/// Protocol magic bytes: "OXID" in little-endian
pub const PROTOCOL_MAGIC: u32 = 0x4F584944;

/// Anti-DPI Discriminators & Junk Preamble Magics
pub const DISCRIMINATOR_OXIDE_DATA: u64 = 0x4F584944_44415441; // "OXIDDATA"
pub const DISCRIMINATOR_OXIDE_CTRL: u64 = 0x4F584944_4354524C; // "OXIDCTRL"
pub const JUNK_PREAMBLE_MAGIC_1: u32 = 0xDEADBEEF;
pub const JUNK_PREAMBLE_MAGIC_2: u32 = 0xBAADF00D;

/// Wire packet header (16 bytes, aligned for SIMD)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct PacketHeader {
    pub magic: u32,
    pub version: u16,
    pub packet_type: u8,
    pub flags: u8,
    pub packet_id: u32,
    pub payload_len: u16,
    pub reserved: u16,
}

impl PacketHeader {
    pub const SIZE: usize = 16;

    pub fn new(packet_type: PacketType, packet_id: u32, payload_len: u16) -> Self {
        Self {
            magic: PROTOCOL_MAGIC,
            version: ProtocolVersion::CURRENT.0,
            packet_type: packet_type as u8,
            flags: 0,
            packet_id,
            payload_len,
            reserved: 0,
        }
    }

    pub fn validate(&self) -> Result<(), OxideError> {
        if self.magic != PROTOCOL_MAGIC {
            return Err(OxideError::Network(
                "Invalid magic bytes in packet header".into(),
            ));
        }
        if self.version < ProtocolVersion::MIN_COMPATIBLE.0
            || self.version > ProtocolVersion::CURRENT.0
        {
            return Err(OxideError::Network(format!(
                "Unsupported protocol version: {}",
                self.version
            )));
        }
        if (self.payload_len as usize) > MAX_PACKET_SIZE - Self::SIZE {
            return Err(OxideError::Network(
                "Payload length exceeds maximum packet size".into(),
            ));
        }
        Ok(())
    }

    pub fn packet_type(&self) -> Result<PacketType, OxideError> {
        PacketType::try_from(self.packet_type)
            .map_err(|_| OxideError::Network("Invalid packet type in header".into()))
    }
}

/// Complete wire packet with header and payload
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WirePacket {
    pub header: PacketHeader,
    pub payload: Vec<u8>,
}

impl WirePacket {
    pub fn new(
        packet_type: PacketType,
        packet_id: u32,
        payload: Vec<u8>,
    ) -> Result<Self, OxideError> {
        if payload.len() > MAX_PACKET_SIZE - PacketHeader::SIZE {
            return Err(OxideError::Network(
                "Payload length too large for wire packet".into(),
            ));
        }
        Ok(Self {
            header: PacketHeader::new(packet_type, packet_id, payload.len() as u16),
            payload,
        })
    }

    pub fn total_len(&self) -> usize {
        PacketHeader::SIZE + self.payload.len()
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(self.total_len());
        buf.put_slice(self.header.as_bytes());
        buf.put_slice(&self.payload);
        buf.freeze().to_vec()
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, OxideError> {
        if bytes.len() < PacketHeader::SIZE {
            return Err(OxideError::Network("Packet too short for header".into()));
        }

        let header = PacketHeader::ref_from_bytes(&bytes[..PacketHeader::SIZE])
            .map_err(|e| OxideError::Network(format!("Header parse error: {e}")))?;
        header.validate()?;

        let payload_len = header.payload_len as usize;
        if bytes.len() != PacketHeader::SIZE + payload_len {
            return Err(OxideError::Network("Packet payload length mismatch".into()));
        }

        Ok(Self {
            header: *header,
            payload: bytes[PacketHeader::SIZE..].to_vec(),
        })
    }

    pub fn packet_type(&self) -> Result<PacketType, OxideError> {
        self.header.packet_type()
    }

    pub fn is_control(&self) -> bool {
        matches!(self.header.packet_type, 0x10..=0x13)
    }
}

/// Batch packet for GRO/GSO aggregation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchPacket {
    pub packets: Vec<WirePacket>,
}

impl BatchPacket {
    pub fn new(packets: Vec<WirePacket>) -> Self {
        Self { packets }
    }

    pub fn total_len(&self) -> usize {
        self.packets.iter().map(|p| p.total_len()).sum()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(self.total_len());
        for packet in &self.packets {
            buf.put_slice(packet.header.as_bytes());
            buf.put_slice(&packet.payload);
        }
        buf.freeze().to_vec()
    }
}

/// Pre-decapsulation packet classification verdict
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreParseVerdict {
    /// Valid data packet with sequence ID and expected payload length
    ValidData { packet_id: u32, payload_len: u16 },
    /// Valid control plane packet
    ValidControl { packet_id: u32, payload_len: u16 },
    /// Recognized junk camouflage frame to be dropped without touching crypto state
    JunkIgnored,
    /// Malformed or unrecognized packet
    Malformed,
}

/// Zero-allocation stateless pre-decapsulation parser
#[inline]
pub fn pre_parse_packet(buffer: &[u8]) -> PreParseVerdict {
    if buffer.len() < PacketHeader::SIZE {
        if buffer.len() >= 4 {
            let magic = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
            if magic == JUNK_PREAMBLE_MAGIC_1 || magic == JUNK_PREAMBLE_MAGIC_2 {
                return PreParseVerdict::JunkIgnored;
            }
        }
        return PreParseVerdict::Malformed;
    }

    let magic = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
    if magic != PROTOCOL_MAGIC {
        let be_magic = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        if be_magic == JUNK_PREAMBLE_MAGIC_1 || be_magic == JUNK_PREAMBLE_MAGIC_2 {
            return PreParseVerdict::JunkIgnored;
        }
        return PreParseVerdict::Malformed;
    }

    let ptype = buffer[6];
    let packet_id = u32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);
    let payload_len = u16::from_le_bytes([buffer[12], buffer[13]]);

    if buffer.len() < PacketHeader::SIZE + (payload_len as usize) {
        return PreParseVerdict::Malformed;
    }

    if (0x10..=0x13).contains(&ptype) {
        PreParseVerdict::ValidControl {
            packet_id,
            payload_len,
        }
    } else {
        PreParseVerdict::ValidData {
            packet_id,
            payload_len,
        }
    }
}

/// 128-bit sliding bitmask replay window (RFC 6479 inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayWindow128 {
    last_seq: u64,
    window: u128,
}

impl Default for ReplayWindow128 {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplayWindow128 {
    pub const fn new() -> Self {
        Self {
            last_seq: 0,
            window: 0,
        }
    }

    /// Validate sequence number and update the sliding window.
    /// Returns `true` if packet is accepted, `false` if duplicate or stale.
    #[inline]
    pub fn check_and_update(&mut self, seq: u64) -> bool {
        if seq == 0 {
            return false;
        }

        if seq > self.last_seq {
            let diff = seq - self.last_seq;
            if diff < 128 {
                self.window = (self.window << diff) | 1;
            } else {
                self.window = 1;
            }
            self.last_seq = seq;
            true
        } else {
            let diff = self.last_seq - seq;
            if diff >= 128 {
                // Older than 128 packets
                return false;
            }
            let bit = 1u128 << diff;
            if (self.window & bit) != 0 {
                // Replay detected
                return false;
            }
            self.window |= bit;
            true
        }
    }

    #[inline]
    pub fn last_sequence(&self) -> u64 {
        self.last_seq
    }
}

/// Control keepalive payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeepalivePayload {
    pub node_id: NodeId,
    pub timestamp: i64,
    pub capabilities: NodeCapabilities,
    pub endpoints: Vec<Endpoint>,
}

/// Path discovery probe payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathDiscoveryPayload {
    pub probe_id: u64,
    pub src_node: NodeId,
    pub dst_node: NodeId,
    pub path_mtu: u16,
    pub timestamp: i64,
}

/// Rekey notice payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RekeyNoticePayload {
    pub node_id: NodeId,
    pub new_session_pub: [u8; 32],
    pub valid_from: i64,
    pub valid_until: i64,
}

/// ACL update payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclUpdatePayload {
    pub version: u64,
    pub rules: Vec<AclRuleWire>,
}

/// Wire format for ACL rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclRuleWire {
    pub action: AclActionWire,
    pub src_identity: Option<[u8; 32]>,
    pub dst_prefix: Option<OverlayPrefix>,
    pub protocol: Option<u8>,
    pub port_range: Option<(u16, u16)>,
    pub direction: AclDirectionWire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AclActionWire {
    Allow,
    Deny,
    Log,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AclDirectionWire {
    Ingress,
    Egress,
    Both,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_packet_roundtrip() {
        let payload = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x42];
        let packet = WirePacket::new(PacketType::Ipv4, 101, payload.clone()).unwrap();
        let bytes = packet.to_bytes();
        let parsed = WirePacket::from_bytes(&bytes).unwrap();
        assert_eq!(packet.payload, parsed.payload);
        assert_eq!(packet.header.packet_id, 101);
    }

    #[test]
    fn test_replay_window_anti_replay() {
        let mut window = ReplayWindow128::new();
        assert!(window.check_and_update(1));
        assert!(window.check_and_update(2));
        assert!(!window.check_and_update(1)); // duplicate
        assert!(window.check_and_update(10));
        assert!(window.check_and_update(5)); // valid out of order
        assert!(!window.check_and_update(5)); // duplicate
        assert!(!window.check_and_update(0)); // invalid 0
    }

    #[test]
    fn test_pre_parse_junk_frame_filter() {
        let junk = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01];
        assert_eq!(pre_parse_packet(&junk), PreParseVerdict::JunkIgnored);

        let packet = WirePacket::new(PacketType::Ipv4, 77, vec![1, 2, 3]).unwrap();
        let bytes = packet.to_bytes();
        assert_eq!(
            pre_parse_packet(&bytes),
            PreParseVerdict::ValidData {
                packet_id: 77,
                payload_len: 3
            }
        );
    }
}
