---
okf_version: "0.2"
type: Module
title: dns
description: "# MagicDNS & OS Resolver Self-Healing Watchdog (`crates/oxide-network/src/dns.rs`)"
resource: crates/oxide-network/src/dns.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:47Z"
concept_id: crates/oxide-network/src/dns
language: rust
---

# dns

# MagicDNS & OS Resolver Self-Healing Watchdog (`crates/oxide-network/src/dns.rs`)

## Docstring

# MagicDNS & OS Resolver Self-Healing Watchdog (`crates/oxide-network/src/dns.rs`)

Provides seamless internal `.oxide` domain name resolution,
mapping human-readable node and service names to overlay IPv4/IPv6 addresses.

## Relationships

| Type | Target |
|------|--------|
| related | [DnsRecord](/crates/oxide-network/src/dns/DnsRecord.md) |
| related | [MagicDnsResolver](/crates/oxide-network/src/dns/MagicDnsResolver.md) |
| related | [default](/crates/oxide-network/src/dns/default.md) |
| related | [default](/crates/oxide-network/src/dns/default.md) |
| related | [new](/crates/oxide-network/src/dns/new.md) |
| related | [normalize_hostname](/crates/oxide-network/src/dns/normalize_hostname.md) |
| related | [register](/crates/oxide-network/src/dns/register.md) |
| related | [unregister](/crates/oxide-network/src/dns/unregister.md) |
| related | [resolve_name](/crates/oxide-network/src/dns/resolve_name.md) |
| related | [reverse_resolve](/crates/oxide-network/src/dns/reverse_resolve.md) |
| related | [list_records](/crates/oxide-network/src/dns/list_records.md) |
| related | [new](/crates/oxide-network/src/dns/new.md) |
| related | [normalize_hostname](/crates/oxide-network/src/dns/normalize_hostname.md) |
| related | [register](/crates/oxide-network/src/dns/register.md) |
| related | [unregister](/crates/oxide-network/src/dns/unregister.md) |
| related | [resolve_name](/crates/oxide-network/src/dns/resolve_name.md) |
| related | [reverse_resolve](/crates/oxide-network/src/dns/reverse_resolve.md) |
| related | [list_records](/crates/oxide-network/src/dns/list_records.md) |
| related | [DnsWatchdogConfig](/crates/oxide-network/src/dns/DnsWatchdogConfig.md) |
| related | [default](/crates/oxide-network/src/dns/default.md) |
| related | [default](/crates/oxide-network/src/dns/default.md) |
| related | [DnsWatchdog](/crates/oxide-network/src/dns/DnsWatchdog.md) |
| related | [new](/crates/oxide-network/src/dns/new.md) |
| related | [config](/crates/oxide-network/src/dns/config.md) |
| related | [check_health](/crates/oxide-network/src/dns/check_health.md) |
| related | [is_healthy](/crates/oxide-network/src/dns/is_healthy.md) |
| related | [new](/crates/oxide-network/src/dns/new.md) |
| related | [config](/crates/oxide-network/src/dns/config.md) |
| related | [check_health](/crates/oxide-network/src/dns/check_health.md) |
| related | [is_healthy](/crates/oxide-network/src/dns/is_healthy.md) |
| related | [test_magic_dns_forward_and_reverse_resolution](/crates/oxide-network/src/dns/test_magic_dns_forward_and_reverse_resolution.md) |
| related | [dashmap](/_dependencies/cargo/dashmap.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
