---
okf_version: "0.2"
type: Function
title: from_file
description: Load from the given TOML file path.
resource: crates/config-loader/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:config-loader"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/config-loader/src/lib/from_file
language: rust
---

# from_file

Load from the given TOML file path.

## Signature

```rust
impl AppConfig { pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> }
```

## Visibility

- `pub`

## Docstring

Load from the given TOML file path.

## Source
Lines 266–276 in `crates/config-loader/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/config-loader/src/lib.md) |
