---
okf_version: "0.2"
type: Function
title: chunk_text
description: ── Text Chunking Helper ──────────────────────────────────────────────────────
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: crates/knowledge/src/client/chunk_text
language: rust
---

# chunk_text

── Text Chunking Helper ──────────────────────────────────────────────────────

## Signature

```rust
fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String>
```

## Docstring

── Text Chunking Helper ──────────────────────────────────────────────────────

## Source
Lines 688–710 in `crates/knowledge/src/client.rs`
