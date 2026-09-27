---
okf_version: "0.2"
type: Function
title: encode
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/encode
language: rust
---

# encode

## Signature

```rust
pub fn encode(bytes: impl AsRef<[u8]>) -> String
```

## Visibility

- `pub`

## Source
Lines 312–318 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
| called_by | [metrics_endpoint](/crates/api/src/routes/health/metrics_endpoint.md) |
| called_by | [fmt](/crates/oxide-network/src/crypto/fmt.md) |
| called_by | [serialize](/crates/oxide-network/src/crypto/serialize.md) |
| called_by | [_send_json](/scripts/serve_ornith/send_json.md) |
