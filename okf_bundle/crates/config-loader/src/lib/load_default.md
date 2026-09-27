---
okf_version: "0.2"
type: Function
title: load_default
description: "Convenience: load `config.toml` relative to the workspace root, then"
resource: crates/config-loader/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:config-loader"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/config-loader/src/lib/load_default
language: rust
---

# load_default

Convenience: load `config.toml` relative to the workspace root, then

## Signature

```rust
impl AppConfig { pub fn load_default() -> Result<Self, ConfigError> }
```

## Visibility

- `pub`

## Docstring

Convenience: load `config.toml` relative to the workspace root, then
fall back to the current directory.  Returns an error only if the file
exists but cannot be parsed; if the file is missing entirely a default
in-memory config is returned so the server still starts.

## Source
Lines 282–299 in `crates/config-loader/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/config-loader/src/lib.md) |
