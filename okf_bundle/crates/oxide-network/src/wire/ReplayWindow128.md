---
okf_version: "0.2"
type: Class
title: ReplayWindow128
description: 128-bit sliding bitmask replay window (RFC 6479 inspired)
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/ReplayWindow128
language: rust
---

# ReplayWindow128

128-bit sliding bitmask replay window (RFC 6479 inspired)

## Signature

```rust
pub struct ReplayWindow128
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

128-bit sliding bitmask replay window (RFC 6479 inspired)
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `last_seq`
- `window`

## Source
Lines 220–223 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
