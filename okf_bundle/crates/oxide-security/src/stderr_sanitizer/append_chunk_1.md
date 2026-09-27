---
okf_version: "0.2"
type: Function
title: append_chunk
description: Append chunk into 16 KiB bounded ring buffer
resource: crates/oxide-security/src/stderr_sanitizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/stderr_sanitizer/append_chunk_1
language: rust
---

# append_chunk

Append chunk into 16 KiB bounded ring buffer

## Signature

```rust
pub fn append_chunk(&self, chunk: &[u8])
```

## Visibility

- `pub`

## Docstring

Append chunk into 16 KiB bounded ring buffer

## Source
Lines 36–44 in `crates/oxide-security/src/stderr_sanitizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stderr_sanitizer](/crates/oxide-security/src/stderr_sanitizer.md) |
