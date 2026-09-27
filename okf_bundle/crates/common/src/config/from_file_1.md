---
okf_version: "0.2"
type: Function
title: from_file
resource: crates/common/src/config.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:common"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/common/src/config/from_file_1
language: rust
---

# from_file

## Signature

```rust
pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError>
```

## Visibility

- `pub`

## Source
Lines 369–379 in `crates/common/src/config.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [config](/crates/common/src/config.md) |
