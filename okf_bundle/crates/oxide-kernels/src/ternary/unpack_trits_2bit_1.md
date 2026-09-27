---
okf_version: "0.2"
type: Function
title: unpack_trits_2bit
description: "Unpack dense trits (encoded as 5 trits per 8-bit byte or 2-bit values) into {-1.0, 0.0, 1.0} scalars."
resource: crates/oxide-kernels/src/ternary.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-kernels"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-kernels/src/ternary/unpack_trits_2bit_1
language: rust
---

# unpack_trits_2bit

Unpack dense trits (encoded as 5 trits per 8-bit byte or 2-bit values) into {-1.0, 0.0, 1.0} scalars.

## Signature

```rust
pub fn unpack_trits_2bit(&self, packed: &[u8], output: &mut [f32]) -> Result<(), OxideError>
```

## Visibility

- `pub`

## Docstring

Unpack dense trits (encoded as 5 trits per 8-bit byte or 2-bit values) into {-1.0, 0.0, 1.0} scalars.

Bit representation:
00 ->  0.0
01 ->  1.0
10 -> -1.0
11 ->  reserved / 0.0

## Source
Lines 37–62 in `crates/oxide-kernels/src/ternary.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ternary](/crates/oxide-kernels/src/ternary.md) |
