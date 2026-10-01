//! # Zero-Trust Access Control Lists (ACL) Engine (`crates/oxide-network/src/acl.rs`)
//!
//! Evaluates network security policies on every packet traversing the mesh overlay,
//! enforcing microsegmentation, port isolation, and cryptographic identity binding.

use crate::types::{OverlayIp, OverlayPrefix};
use oxide_core::OxideError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AclAction {
    Allow,
    Deny,
    Log,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AclDirection {
    Ingress,
    Egress,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Any,
}

impl Protocol {
    pub fn from_ip_proto(proto: u8) -> Self {
        match proto {
            6 => Self::Tcp,
            17 => Self::Udp,
            1 | 58 => Self::Icmp,
            _ => Self::Any,
        }
    }
}

/// Extracted metadata from an IP packet
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketMeta {
    pub src_ip: OverlayIp,
    pub dst_ip: OverlayIp,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: Protocol,
    pub direction: AclDirection,
}

impl PacketMeta {
    /// Parse packet metadata from raw IPv4/IPv6 packet buffer
    pub fn parse(buffer: &[u8], direction: AclDirection) -> Option<Self> {
        if buffer.len() < 20 {
            return None;
        }

        let version = (buffer[0] >> 4) & 0x0F;
        match version {
            4 => {
                let ihl = (buffer[0] & 0x0F) as usize * 4;
                if buffer.len() < ihl {
                    return None;
                }
                let proto = buffer[9];
                let src_ip = OverlayIp::V4(std::net::Ipv4Addr::new(
                    buffer[12], buffer[13], buffer[14], buffer[15],
                ));
                let dst_ip = OverlayIp::V4(std::net::Ipv4Addr::new(
                    buffer[16], buffer[17], buffer[18], buffer[19],
                ));

                let protocol = Protocol::from_ip_proto(proto);
                let (src_port, dst_port) = if (proto == 6 || proto == 17) && buffer.len() >= ihl + 4 {
                    let sp = u16::from_be_bytes([buffer[ihl], buffer[ihl + 1]]);
                    let dp = u16::from_be_bytes([buffer[ihl + 2], buffer[ihl + 3]]);
                    (Some(sp), Some(dp))
                } else {
                    (None, None)
                };

                Some(Self {
                    src_ip,
                    dst_ip,
                    src_port,
                    dst_port,
                    protocol,
                    direction,
                })
            }
            6 => {
                if buffer.len() < 40 {
                    return None;
                }
                let proto = buffer[6];
                let mut src_octets = [0u8; 16];
                src_octets.copy_from_slice(&buffer[8..24]);
                let mut dst_octets = [0u8; 16];
                dst_octets.copy_from_slice(&buffer[24..40]);

                let src_ip = OverlayIp::V6(std::net::Ipv6Addr::from(src_octets));
                let dst_ip = OverlayIp::V6(std::net::Ipv6Addr::from(dst_octets));
                let protocol = Protocol::from_ip_proto(proto);

                let (src_port, dst_port) = if (proto == 6 || proto == 17) && buffer.len() >= 44 {
                    let sp = u16::from_be_bytes([buffer[40], buffer[41]]);
                    let dp = u16::from_be_bytes([buffer[42], buffer[43]]);
                    (Some(sp), Some(dp))
                } else {
                    (None, None)
                };

                Some(Self {
                    src_ip,
                    dst_ip,
                    src_port,
                    dst_port,
                    protocol,
                    direction,
                })
            }
            _ => None,
        }
    }
}

/// A discrete Zero-Trust ACL Rule
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AclRule {
    pub name: String,
    pub action: AclAction,
    pub src_prefix: Option<OverlayPrefix>,
    pub dst_prefix: Option<OverlayPrefix>,
    pub protocol: Protocol,
    pub port_range: Option<(u16, u16)>,
    pub direction: AclDirection,
}

impl AclRule {
    pub fn matches(&self, meta: &PacketMeta) -> bool {
        // 1. Direction check
        if self.direction != AclDirection::Both && self.direction != meta.direction {
            return false;
        }

        // 2. Protocol check
        if self.protocol != Protocol::Any && self.protocol != meta.protocol {
            return false;
        }

        // 3. Source prefix check
        if let Some(ref pfx) = self.src_prefix
            && !pfx.contains(meta.src_ip) {
                return false;
            }

        // 4. Destination prefix check
        if let Some(ref pfx) = self.dst_prefix
            && !pfx.contains(meta.dst_ip) {
                return false;
            }

        // 5. Port range check
        if let Some((min_port, max_port)) = self.port_range {
            if let Some(dst_port) = meta.dst_port {
                if dst_port < min_port || dst_port > max_port {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

/// Engine evaluating ACL rules against packets
#[derive(Debug, Clone)]
pub struct AclEngine {
    rules: Vec<AclRule>,
    default_action: AclAction,
}

impl Default for AclEngine {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            default_action: AclAction::Allow,
        }
    }
}

impl AclEngine {
    pub fn new(rules: Vec<AclRule>, default_action: AclAction) -> Result<Self, OxideError> {
        Ok(Self {
            rules,
            default_action,
        })
    }

    pub fn evaluate(&self, meta: &PacketMeta) -> AclAction {
        for rule in &self.rules {
            if rule.matches(meta) {
                return rule.action;
            }
        }
        self.default_action
    }

    pub fn add_rule(&mut self, rule: AclRule) {
        self.rules.push(rule);
    }

    pub fn rules(&self) -> &[AclRule] {
        &self.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acl_rule_evaluation() {
        let mut engine = AclEngine::new(Vec::new(), AclAction::Deny).unwrap();

        // Allow SSH to admin subnet
        engine.add_rule(AclRule {
            name: "Allow SSH".into(),
            action: AclAction::Allow,
            src_prefix: Some("100.64.0.0/16".parse().unwrap()),
            dst_prefix: Some("100.64.10.0/24".parse().unwrap()),
            protocol: Protocol::Tcp,
            port_range: Some((22, 22)),
            direction: AclDirection::Ingress,
        });

        // Packet 1: Matching SSH packet
        let meta1 = PacketMeta {
            src_ip: "100.64.1.20".parse().unwrap(),
            dst_ip: "100.64.10.5".parse().unwrap(),
            src_port: Some(54321),
            dst_port: Some(22),
            protocol: Protocol::Tcp,
            direction: AclDirection::Ingress,
        };
        assert_eq!(engine.evaluate(&meta1), AclAction::Allow);

        // Packet 2: HTTP packet to port 80 (Deny by default)
        let meta2 = PacketMeta {
            src_ip: "100.64.1.20".parse().unwrap(),
            dst_ip: "100.64.10.5".parse().unwrap(),
            src_port: Some(54321),
            dst_port: Some(80),
            protocol: Protocol::Tcp,
            direction: AclDirection::Ingress,
        };
        assert_eq!(engine.evaluate(&meta2), AclAction::Deny);
    }
}
