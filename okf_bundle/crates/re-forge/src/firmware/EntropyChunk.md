---
okf_version: "0.2"
type: Class
title: EntropyChunk
description: Shannon Entropy Scanner for detecting compressed/encrypted firmware payloads
resource: crates/re-forge/src/firmware.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/re-forge/src/firmware/EntropyChunk
language: rust
---

# EntropyChunk

Shannon Entropy Scanner for detecting compressed/encrypted firmware payloads

## Signature

```rust
pub struct EntropyChunk
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Shannon Entropy Scanner for detecting compressed/encrypted firmware payloads
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `offset`
- `size`
- `entropy`
- `classification`

## Source
Lines 77–82 in `crates/re-forge/src/firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [firmware](/crates/re-forge/src/firmware.md) |
