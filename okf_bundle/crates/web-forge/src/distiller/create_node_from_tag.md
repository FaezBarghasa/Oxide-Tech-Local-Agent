---
okf_version: "0.2"
type: Function
title: create_node_from_tag
resource: crates/web-forge/src/distiller.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:37:19Z"
concept_id: crates/web-forge/src/distiller/create_node_from_tag
language: rust
---

# create_node_from_tag

## Signature

```rust
impl DomDistiller { fn create_node_from_tag(
        tree: &mut AXTree,
        current_id: &mut u64,
        tag: &str,
        attrs: &str,
        inner_text: &str,
    ) }
```

## Source
Lines 214–278 in `crates/web-forge/src/distiller.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distiller](/crates/web-forge/src/distiller.md) |
