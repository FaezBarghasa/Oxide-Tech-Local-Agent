---
okf_version: "0.2"
type: Function
title: compact_compiler_log
description: "Strips excessive noise and formats compiler outputs to preserve key diagnostics while saving 60-80% tokens."
resource: crates/rag-pipeline/src/okf.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/rag-pipeline/src/okf/compact_compiler_log
language: rust
---

# compact_compiler_log

Strips excessive noise and formats compiler outputs to preserve key diagnostics while saving 60-80% tokens.

## Signature

```rust
impl StenoCompactor { pub fn compact_compiler_log(raw_log: &str) -> String }
```

## Visibility

- `pub`

## Docstring

Strips excessive noise and formats compiler outputs to preserve key diagnostics while saving 60-80% tokens.

## Source
Lines 42–61 in `crates/rag-pipeline/src/okf.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [okf](/crates/rag-pipeline/src/okf.md) |
