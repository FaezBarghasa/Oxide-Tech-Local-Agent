---
okf_version: "0.2"
type: Class
title: Document
description: "[derive(Debug, Serialize, Deserialize, Clone)]"
resource: crates/oxide-state/src/schema.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/schema/Document
language: rust
---

# Document

[derive(Debug, Serialize, Deserialize, Clone)]

## Signature

```rust
pub struct Document
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `id`
- `title`
- `source`
- `content`
- `created_at`

## Source
Lines 70–77 in `crates/oxide-state/src/schema.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schema](/crates/oxide-state/src/schema.md) |
| called_by | [from_path](/crates/tree-sitter-service/src/languages/from_path.md) |
