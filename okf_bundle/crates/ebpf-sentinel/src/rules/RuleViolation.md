---
okf_version: "0.2"
type: Class
title: RuleViolation
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/ebpf-sentinel/src/rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:ebpf-sentinel"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:51:59Z"
concept_id: crates/ebpf-sentinel/src/rules/RuleViolation
language: rust
---

# RuleViolation

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct RuleViolation
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `pid`
- `syscall_name`
- `reason`
- `timestamp_epoch_ms`

## Source
Lines 32–37 in `crates/ebpf-sentinel/src/rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/ebpf-sentinel/src/rules.md) |
