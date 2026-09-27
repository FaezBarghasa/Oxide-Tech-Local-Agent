---
okf_version: "0.2"
type: Class
title: ContentPart
description: "[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]"
resource: crates/oxide-core/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T13:03:50Z"
concept_id: crates/oxide-core/src/lib/ContentPart
language: rust
---

# ContentPart

[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

## Signature

```rust
pub enum ContentPart
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)`
- `serde(tag = "type", rename_all = "snake_case")`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
[serde(tag = "type", rename_all = "snake_case")]

## Methods

- `text`
- `image_url`

## Source
Lines 23–26 in `crates/oxide-core/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-core/src/lib.md) |
