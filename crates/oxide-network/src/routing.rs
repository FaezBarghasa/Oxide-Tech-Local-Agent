//! # High-Performance Lock-Free RCU Routing Fabric (`crates/oxide-network/src/routing.rs`)
//!
//! Provides Radix Longest-Prefix Match (LPM) routing, L1 direct-mapped caching,
//! and lock-free Read-Copy-Update (RCU) table updates with zero packet stall.

use crate::types::{NodeId, OverlayIp, OverlayPrefix};
use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

/// Target destination for routed overlay packets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RouteTarget {
    /// Direct P2P transport connection to peer node
    DirectPeer {
        node_id: NodeId,
        endpoint: SocketAddr,
    },
    /// Next-hop subnet router gateway
    SubnetRouter {
        router_id: NodeId,
        gateway_ip: OverlayIp,
    },
    /// Relay / DERP fallback node
    Relay { relay_id: NodeId },
    /// Local machine loopback
    LocalLoopback,
}

/// A discrete route table entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteEntry {
    pub prefix: OverlayPrefix,
    pub target: RouteTarget,
    pub metric: u32,
    pub pmtu: u16,
}

/// Radix LPM Routing Table
#[derive(Debug, Clone, Default)]
pub struct RadixRoutingTable {
    entries: Vec<RouteEntry>,
}

impl RadixRoutingTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, entry: RouteEntry) {
        // Remove existing entry for exact same prefix
        self.entries.retain(|e| e.prefix != entry.prefix);
        self.entries.push(entry);
        // Sort by prefix length descending (longest prefix match) then by metric ascending
        self.entries.sort_by(|a, b| {
            b.prefix
                .prefix_len
                .cmp(&a.prefix.prefix_len)
                .then_with(|| a.metric.cmp(&b.metric))
        });
    }

    pub fn remove(&mut self, prefix: &OverlayPrefix) -> bool {
        let initial_len = self.entries.len();
        self.entries.retain(|e| &e.prefix != prefix);
        self.entries.len() != initial_len
    }

    pub fn lookup(&self, ip: OverlayIp) -> Option<RouteTarget> {
        for entry in &self.entries {
            if entry.prefix.contains(ip) {
                return Some(entry.target);
            }
        }
        None
    }

    pub fn list_routes(&self) -> Vec<RouteEntry> {
        self.entries.clone()
    }
}

/// L1 Direct-Mapped Cache for zero-allocation route resolution
pub struct L1DirectMappedCache {
    slots: [(u32, Option<RouteTarget>); 256],
}

impl Default for L1DirectMappedCache {
    fn default() -> Self {
        Self::new()
    }
}

impl L1DirectMappedCache {
    pub fn new() -> Self {
        Self {
            slots: [(0, None); 256],
        }
    }

    #[inline]
    fn hash_ip(ip: OverlayIp) -> usize {
        match ip {
            OverlayIp::V4(v4) => (u32::from(v4) % 256) as usize,
            OverlayIp::V6(v6) => {
                let octets = v6.octets();
                (octets[15] as usize) ^ (octets[14] as usize)
            }
        }
    }

    #[inline]
    pub fn get(&self, ip: OverlayIp) -> Option<RouteTarget> {
        let slot_idx = Self::hash_ip(ip);
        let (stored_key, target) = self.slots[slot_idx];
        let current_key = match ip {
            OverlayIp::V4(v4) => u32::from(v4),
            OverlayIp::V6(v6) => {
                let oct = v6.octets();
                u32::from_be_bytes([oct[12], oct[13], oct[14], oct[15]])
            }
        };

        if stored_key == current_key {
            target
        } else {
            None
        }
    }

    #[inline]
    pub fn put(&mut self, ip: OverlayIp, target: RouteTarget) {
        let slot_idx = Self::hash_ip(ip);
        let key = match ip {
            OverlayIp::V4(v4) => u32::from(v4),
            OverlayIp::V6(v6) => {
                let oct = v6.octets();
                u32::from_be_bytes([oct[12], oct[13], oct[14], oct[15]])
            }
        };
        self.slots[slot_idx] = (key, Some(target));
    }

    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            *slot = (0, None);
        }
    }
}

/// Read-Copy-Update (RCU) Router for lock-free parallel packet forwarding
pub struct RcuRouter {
    table: ArcSwap<RadixRoutingTable>,
}

impl Default for RcuRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl RcuRouter {
    pub fn new() -> Self {
        Self {
            table: ArcSwap::from_pointee(RadixRoutingTable::new()),
        }
    }

    #[inline]
    pub fn lookup(&self, ip: OverlayIp) -> Option<RouteTarget> {
        let guard = self.table.load();
        guard.lookup(ip)
    }

    pub fn insert_route(&self, entry: RouteEntry) {
        let current = self.table.load_full();
        let mut new_table = (*current).clone();
        new_table.insert(entry);
        self.table.store(Arc::new(new_table));
    }

    pub fn remove_route(&self, prefix: &OverlayPrefix) -> bool {
        let current = self.table.load_full();
        let mut new_table = (*current).clone();
        let removed = new_table.remove(prefix);
        if removed {
            self.table.store(Arc::new(new_table));
        }
        removed
    }

    pub fn get_routes(&self) -> Vec<RouteEntry> {
        self.table.load().list_routes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_lpm_and_rcu_router() {
        let router = RcuRouter::new();
        let node_id = NodeId::new();
        let endpoint = "192.168.1.100:9000".parse().unwrap();

        // Specific /32 route
        let p1: OverlayPrefix = "100.64.1.5/32".parse().unwrap();
        router.insert_route(RouteEntry {
            prefix: p1,
            target: RouteTarget::DirectPeer { node_id, endpoint },
            metric: 10,
            pmtu: 1400,
        });

        // Catch-all /16 subnet route
        let p2: OverlayPrefix = "100.64.0.0/16".parse().unwrap();
        router.insert_route(RouteEntry {
            prefix: p2,
            target: RouteTarget::Relay { relay_id: node_id },
            metric: 100,
            pmtu: 1280,
        });

        // Query exact host match -> DirectPeer
        let ip_host: OverlayIp = "100.64.1.5".parse().unwrap();
        assert_eq!(
            router.lookup(ip_host),
            Some(RouteTarget::DirectPeer { node_id, endpoint })
        );

        // Query other host in /16 -> Relay (LPM fallback)
        let ip_other: OverlayIp = "100.64.2.9".parse().unwrap();
        assert_eq!(
            router.lookup(ip_other),
            Some(RouteTarget::Relay { relay_id: node_id })
        );

        // Query external IP -> None
        let ip_ext: OverlayIp = "192.168.1.1".parse().unwrap();
        assert_eq!(router.lookup(ip_ext), None);
    }
}
