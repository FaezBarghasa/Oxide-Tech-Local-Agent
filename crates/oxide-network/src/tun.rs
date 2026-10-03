//! # Virtual Network Interface Engine (TUN) & MSS Clamping (`crates/oxide-network/src/tun.rs`)
//!
//! Provides virtual network device abstractions and dynamic TCP MSS clamping
//! to prevent packet fragmentation over encapsulated overlay tunnels.

use crate::types::OverlayIp;
use oxide_core::OxideError;
use serde::{Deserialize, Serialize};

/// Configuration parameters for virtual TUN device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunConfig {
    pub name: String,
    pub ip: OverlayIp,
    pub netmask: String,
    pub mtu: u16,
    pub multi_queue: bool,
}

impl Default for TunConfig {
    fn default() -> Self {
        Self {
            name: "oxide0".into(),
            ip: OverlayIp::default_v4(),
            netmask: "255.192.0.0".into(), // /10
            mtu: 1420,
            multi_queue: true,
        }
    }
}

/// In-place dynamic TCP MSS (Maximum Segment Size) Clamping
/// Modifies SYN / SYN-ACK packets to enforce MSS <= max_mss
pub fn clamp_tcp_mss(packet: &mut [u8], max_mss: u16) -> Result<bool, OxideError> {
    if packet.len() < 40 {
        return Ok(false);
    }

    let version = (packet[0] >> 4) & 0x0F;
    let (ip_header_len, ip_proto) = match version {
        4 => {
            let ihl = (packet[0] & 0x0F) as usize * 4;
            if packet.len() < ihl + 20 {
                return Ok(false);
            }
            (ihl, packet[9])
        }
        6 => {
            if packet.len() < 40 + 20 {
                return Ok(false);
            }
            (40, packet[6])
        }
        _ => return Ok(false),
    };

    // Check if transport protocol is TCP (6)
    if ip_proto != 6 {
        return Ok(false);
    }

    let tcp_slice = &mut packet[ip_header_len..];
    let flags = tcp_slice[13];
    let is_syn = (flags & 0x02) != 0;

    if !is_syn {
        return Ok(false);
    }

    let data_offset = ((tcp_slice[12] >> 4) as usize) * 4;
    if tcp_slice.len() < data_offset || data_offset <= 20 {
        return Ok(false);
    }

    // Inspect TCP options for MSS option (kind = 2, length = 4)
    let mut opt_idx = 20;
    let mut modified = false;

    while opt_idx < data_offset {
        let kind = tcp_slice[opt_idx];
        if kind == 0 {
            break; // End of options
        }
        if kind == 1 {
            opt_idx += 1; // NOP
            continue;
        }
        if opt_idx + 1 >= data_offset {
            break;
        }
        let len = tcp_slice[opt_idx + 1] as usize;
        if len < 2 || opt_idx + len > data_offset {
            break;
        }

        if kind == 2 && len == 4 {
            let current_mss = u16::from_be_bytes([tcp_slice[opt_idx + 2], tcp_slice[opt_idx + 3]]);
            if current_mss > max_mss {
                let new_mss_bytes = max_mss.to_be_bytes();
                tcp_slice[opt_idx + 2] = new_mss_bytes[0];
                tcp_slice[opt_idx + 3] = new_mss_bytes[1];
                modified = true;
                // Recompute TCP checksum
                recompute_tcp_checksum(packet, ip_header_len, version);
            }
            break;
        }
        opt_idx += len;
    }

    Ok(modified)
}

/// Helper function to recompute checksum
fn recompute_tcp_checksum(packet: &mut [u8], ip_header_len: usize, version: u8) {
    if version == 4 {
        // Zero existing checksum field in TCP header (offset 16-17 in TCP header)
        packet[ip_header_len + 16] = 0;
        packet[ip_header_len + 17] = 0;

        let src_ip = [packet[12], packet[13], packet[14], packet[15]];
        let dst_ip = [packet[16], packet[17], packet[18], packet[19]];
        let tcp_len = (packet.len() - ip_header_len) as u16;

        let mut sum: u32 = 0;
        // Pseudo-header
        sum += u16::from_be_bytes([src_ip[0], src_ip[1]]) as u32;
        sum += u16::from_be_bytes([src_ip[2], src_ip[3]]) as u32;
        sum += u16::from_be_bytes([dst_ip[0], dst_ip[1]]) as u32;
        sum += u16::from_be_bytes([dst_ip[2], dst_ip[3]]) as u32;
        sum += 6; // Protocol TCP
        sum += tcp_len as u32;

        // TCP payload
        for chunk in packet[ip_header_len..].chunks(2) {
            let word = if chunk.len() == 2 {
                u16::from_be_bytes([chunk[0], chunk[1]])
            } else {
                u16::from_be_bytes([chunk[0], 0])
            };
            sum += word as u32;
        }

        while (sum >> 16) > 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        let checksum = !(sum as u16);
        let cs_bytes = checksum.to_be_bytes();
        packet[ip_header_len + 16] = cs_bytes[0];
        packet[ip_header_len + 17] = cs_bytes[1];
    }
}

/// Virtual TUN Device Controller
pub struct TunDevice {
    config: TunConfig,
    active: bool,
}

impl TunDevice {
    pub fn new(config: TunConfig) -> Result<Self, OxideError> {
        Ok(Self {
            config,
            active: true,
        })
    }

    pub fn config(&self) -> &TunConfig {
        &self.config
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_mss_clamping() {
        // Construct minimal synthetic IPv4 TCP SYN packet with MSS=1460
        let mut packet = vec![
            0x45, 0x00, 0x00, 0x3C, // IPv4, IHL=5, Total Len=60
            0x12, 0x34, 0x40, 0x00, // ID, Flags (DF)
            0x40, 0x06, 0x00, 0x00, // TTL=64, Protocol=6 (TCP), IP Checksum
            100, 64, 0, 1, // Src IP
            100, 64, 0, 2, // Dst IP
            // TCP Header (24 bytes, data offset = 6)
            0x04, 0xD2, 0x00, 0x50, // Src Port 1234, Dst Port 80
            0x00, 0x00, 0x00, 0x01, // Seq num
            0x00, 0x00, 0x00, 0x00, // Ack num
            0x60, 0x02, 0x72, 0x10, // Data offset 6, SYN flag, Window
            0x00, 0x00, 0x00, 0x00, // Checksum, Urgent pointer
            // Options: MSS (kind=2, len=4, value=1460 (0x05B4))
            0x02, 0x04, 0x05, 0xB4,
        ];

        let clamped = clamp_tcp_mss(&mut packet, 1360).unwrap();
        assert!(clamped);

        // Verify MSS option was clamped to 1360 (0x0550)
        assert_eq!(packet[42], 0x05);
        assert_eq!(packet[43], 0x50);
    }
}
