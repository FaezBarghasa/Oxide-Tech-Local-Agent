---
okf_version: "0.2"
type: Class
title: AtomicFileSnapshot
description: An atomic snapshot of file contents before agentic modification
resource: crates/verifier/src/checkpoint.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/checkpoint/AtomicFileSnapshot
language: rust
---

# AtomicFileSnapshot

An atomic snapshot of file contents before agentic modification

## Signature

```rust
pub struct AtomicFileSnapshot
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

An atomic snapshot of file contents before agentic modification
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `file_path`
- `original_content`

## Source
Lines 8–11 in `crates/verifier/src/checkpoint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [checkpoint](/crates/verifier/src/checkpoint.md) |
