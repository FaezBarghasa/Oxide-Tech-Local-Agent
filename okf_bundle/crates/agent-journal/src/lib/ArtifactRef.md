---
okf_version: "0.2"
type: Class
title: ArtifactRef
description: "Typed artifact reference — replaces the bare `Option<String>` result field."
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/ArtifactRef
language: rust
---

# ArtifactRef

Typed artifact reference — replaces the bare `Option<String>` result field.

## Signature

```rust
pub struct ArtifactRef
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, SurrealValue)`

## Visibility

- `pub`

## Docstring

Typed artifact reference — replaces the bare `Option<String>` result field.
[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]

## Methods

- `label`
- `path`
- `inline`

## Source
Lines 36–43 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
