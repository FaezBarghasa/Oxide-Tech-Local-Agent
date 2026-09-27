---
okf_version: "0.2"
type: Function
title: check_and_update
description: Validate sequence number and update the sliding window.
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/check_and_update
language: rust
---

# check_and_update

Validate sequence number and update the sliding window.

## Signature

```rust
impl ReplayWindow128 { pub fn check_and_update(&mut self, seq: u64) -> bool }
```

## Visibility

- `pub`

## Docstring

Validate sequence number and update the sliding window.
Returns `true` if packet is accepted, `false` if duplicate or stale.
[inline]

## Source
Lines 242–270 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
