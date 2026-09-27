---
okf_version: "0.2"
type: Function
title: distill_axtree
description: "Converts an `AXTree` into structured, token-efficient Markdown."
resource: crates/web-forge/src/distiller.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:19Z"
concept_id: crates/web-forge/src/distiller/distill_axtree
language: rust
---

# distill_axtree

Converts an `AXTree` into structured, token-efficient Markdown.

## Signature

```rust
impl DomDistiller { pub fn distill_axtree(tree: &AXTree) -> String }
```

## Visibility

- `pub`

## Docstring

Converts an `AXTree` into structured, token-efficient Markdown.

## Source
Lines 82–144 in `crates/web-forge/src/distiller.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distiller](/crates/web-forge/src/distiller.md) |
