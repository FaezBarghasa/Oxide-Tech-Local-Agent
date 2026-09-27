---
okf_version: "0.2"
type: Function
title: get_sanitized_tail
description: Get sanitized tail as string with secret patterns scrubbed
resource: crates/oxide-security/src/stderr_sanitizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/stderr_sanitizer/get_sanitized_tail
language: rust
---

# get_sanitized_tail

Get sanitized tail as string with secret patterns scrubbed

## Signature

```rust
impl StderrSanitizer { pub fn get_sanitized_tail(&self) -> String }
```

## Visibility

- `pub`

## Docstring

Get sanitized tail as string with secret patterns scrubbed

## Source
Lines 47–59 in `crates/oxide-security/src/stderr_sanitizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stderr_sanitizer](/crates/oxide-security/src/stderr_sanitizer.md) |
