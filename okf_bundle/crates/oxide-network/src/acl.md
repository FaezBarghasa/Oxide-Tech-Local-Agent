---
okf_version: "0.2"
type: Module
title: acl
description: "# Zero-Trust Access Control Lists (ACL) Engine (`crates/oxide-network/src/acl.rs`)"
resource: crates/oxide-network/src/acl.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:47Z"
concept_id: crates/oxide-network/src/acl
language: rust
---

# acl

# Zero-Trust Access Control Lists (ACL) Engine (`crates/oxide-network/src/acl.rs`)

## Docstring

# Zero-Trust Access Control Lists (ACL) Engine (`crates/oxide-network/src/acl.rs`)

Evaluates network security policies on every packet traversing the mesh overlay,
enforcing microsegmentation, port isolation, and cryptographic identity binding.

## Relationships

| Type | Target |
|------|--------|
| related | [AclAction](/crates/oxide-network/src/acl/AclAction.md) |
| related | [AclDirection](/crates/oxide-network/src/acl/AclDirection.md) |
| related | [Protocol](/crates/oxide-network/src/acl/Protocol.md) |
| related | [from_ip_proto](/crates/oxide-network/src/acl/from_ip_proto.md) |
| related | [from_ip_proto](/crates/oxide-network/src/acl/from_ip_proto.md) |
| related | [PacketMeta](/crates/oxide-network/src/acl/PacketMeta.md) |
| related | [parse](/crates/oxide-network/src/acl/parse.md) |
| related | [parse](/crates/oxide-network/src/acl/parse.md) |
| related | [AclRule](/crates/oxide-network/src/acl/AclRule.md) |
| related | [matches](/crates/oxide-network/src/acl/matches.md) |
| related | [matches](/crates/oxide-network/src/acl/matches.md) |
| related | [AclEngine](/crates/oxide-network/src/acl/AclEngine.md) |
| related | [default](/crates/oxide-network/src/acl/default.md) |
| related | [default](/crates/oxide-network/src/acl/default.md) |
| related | [new](/crates/oxide-network/src/acl/new.md) |
| related | [evaluate](/crates/oxide-network/src/acl/evaluate.md) |
| related | [add_rule](/crates/oxide-network/src/acl/add_rule.md) |
| related | [rules](/crates/oxide-network/src/acl/rules.md) |
| related | [new](/crates/oxide-network/src/acl/new.md) |
| related | [evaluate](/crates/oxide-network/src/acl/evaluate.md) |
| related | [add_rule](/crates/oxide-network/src/acl/add_rule.md) |
| related | [rules](/crates/oxide-network/src/acl/rules.md) |
| related | [test_acl_rule_evaluation](/crates/oxide-network/src/acl/test_acl_rule_evaluation.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
