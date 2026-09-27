---
okf_version: "0.2"
type: Function
title: build_uds_bridge_client
resource: crates/mcp-clients/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-clients"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/mcp-clients/src/lib/build_uds_bridge_client
language: rust
---

# build_uds_bridge_client

## Signature

```rust
pub fn build_uds_bridge_client() -> std::result::Result<BridgeServiceClient<Channel>, anyhow::Error>
```

## Visibility

- `pub`

## Source
Lines 112–121 in `crates/mcp-clients/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-clients/src/lib.md) |
