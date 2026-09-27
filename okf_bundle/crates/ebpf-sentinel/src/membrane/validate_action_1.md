---
okf_version: "0.2"
type: Function
title: validate_action
description: Intercept and validate a process action against kernel-level safety rules
resource: crates/ebpf-sentinel/src/membrane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:ebpf-sentinel"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:54:15Z"
concept_id: crates/ebpf-sentinel/src/membrane/validate_action_1
language: rust
---

# validate_action

Intercept and validate a process action against kernel-level safety rules

## Signature

```rust
pub fn validate_action(
        &self,
        pid: u32,
        category: &SyscallCategory,
        target: &str,
    ) -> Result<(), RuleViolation>
```

## Visibility

- `pub`

## Docstring

Intercept and validate a process action against kernel-level safety rules

## Source
Lines 32–66 in `crates/ebpf-sentinel/src/membrane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [membrane](/crates/ebpf-sentinel/src/membrane.md) |
