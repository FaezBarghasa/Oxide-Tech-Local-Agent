---
okf_version: "0.2"
type: Class
title: WebScript
description: A serialized sequence of web actions forming an end-to-end user journey or test script.
resource: crates/web-forge/src/codegen.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:21Z"
concept_id: crates/web-forge/src/codegen/WebScript
language: rust
---

# WebScript

A serialized sequence of web actions forming an end-to-end user journey or test script.

## Signature

```rust
pub struct WebScript
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A serialized sequence of web actions forming an end-to-end user journey or test script.
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `name`
- `description`
- `actions`

## Source
Lines 40–44 in `crates/web-forge/src/codegen.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codegen](/crates/web-forge/src/codegen.md) |
