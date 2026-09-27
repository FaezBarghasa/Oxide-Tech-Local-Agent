---
okf_version: "0.2"
type: Module
title: routing
description: "# High-Performance Lock-Free RCU Routing Fabric (`crates/oxide-network/src/routing.rs`)"
resource: crates/oxide-network/src/routing.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:25Z"
concept_id: crates/oxide-network/src/routing
language: rust
---

# routing

# High-Performance Lock-Free RCU Routing Fabric (`crates/oxide-network/src/routing.rs`)

## Docstring

# High-Performance Lock-Free RCU Routing Fabric (`crates/oxide-network/src/routing.rs`)

Provides Radix Longest-Prefix Match (LPM) routing, L1 direct-mapped caching,
and lock-free Read-Copy-Update (RCU) table updates with zero packet stall.

## Relationships

| Type | Target |
|------|--------|
| related | [RouteTarget](/crates/oxide-network/src/routing/RouteTarget.md) |
| related | [RouteEntry](/crates/oxide-network/src/routing/RouteEntry.md) |
| related | [RadixRoutingTable](/crates/oxide-network/src/routing/RadixRoutingTable.md) |
| related | [new](/crates/oxide-network/src/routing/new.md) |
| related | [insert](/crates/oxide-network/src/routing/insert.md) |
| related | [remove](/crates/oxide-network/src/routing/remove.md) |
| related | [lookup](/crates/oxide-network/src/routing/lookup.md) |
| related | [list_routes](/crates/oxide-network/src/routing/list_routes.md) |
| related | [new](/crates/oxide-network/src/routing/new.md) |
| related | [insert](/crates/oxide-network/src/routing/insert.md) |
| related | [remove](/crates/oxide-network/src/routing/remove.md) |
| related | [lookup](/crates/oxide-network/src/routing/lookup.md) |
| related | [list_routes](/crates/oxide-network/src/routing/list_routes.md) |
| related | [L1DirectMappedCache](/crates/oxide-network/src/routing/L1DirectMappedCache.md) |
| related | [default](/crates/oxide-network/src/routing/default.md) |
| related | [default](/crates/oxide-network/src/routing/default.md) |
| related | [new](/crates/oxide-network/src/routing/new.md) |
| related | [hash_ip](/crates/oxide-network/src/routing/hash_ip.md) |
| related | [get](/crates/oxide-network/src/routing/get.md) |
| related | [put](/crates/oxide-network/src/routing/put.md) |
| related | [clear](/crates/oxide-network/src/routing/clear.md) |
| related | [new](/crates/oxide-network/src/routing/new.md) |
| related | [hash_ip](/crates/oxide-network/src/routing/hash_ip.md) |
| related | [get](/crates/oxide-network/src/routing/get.md) |
| related | [put](/crates/oxide-network/src/routing/put.md) |
| related | [clear](/crates/oxide-network/src/routing/clear.md) |
| related | [RcuRouter](/crates/oxide-network/src/routing/RcuRouter.md) |
| related | [default](/crates/oxide-network/src/routing/default.md) |
| related | [default](/crates/oxide-network/src/routing/default.md) |
| related | [new](/crates/oxide-network/src/routing/new.md) |
| related | [lookup](/crates/oxide-network/src/routing/lookup.md) |
| related | [insert_route](/crates/oxide-network/src/routing/insert_route.md) |
| related | [remove_route](/crates/oxide-network/src/routing/remove_route.md) |
| related | [get_routes](/crates/oxide-network/src/routing/get_routes.md) |
| related | [new](/crates/oxide-network/src/routing/new.md) |
| related | [lookup](/crates/oxide-network/src/routing/lookup.md) |
| related | [insert_route](/crates/oxide-network/src/routing/insert_route.md) |
| related | [remove_route](/crates/oxide-network/src/routing/remove_route.md) |
| related | [get_routes](/crates/oxide-network/src/routing/get_routes.md) |
| related | [test_radix_lpm_and_rcu_router](/crates/oxide-network/src/routing/test_radix_lpm_and_rcu_router.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
