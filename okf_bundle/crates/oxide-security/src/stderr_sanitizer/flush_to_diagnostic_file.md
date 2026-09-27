---
okf_version: "0.2"
type: Function
title: flush_to_diagnostic_file
description: Flush sanitized tail to isolated 0600 file
resource: crates/oxide-security/src/stderr_sanitizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/stderr_sanitizer/flush_to_diagnostic_file
language: rust
---

# flush_to_diagnostic_file

Flush sanitized tail to isolated 0600 file

## Signature

```rust
impl StderrSanitizer { pub fn flush_to_diagnostic_file(&self) -> std::io::Result<()> }
```

## Visibility

- `pub`

## Docstring

Flush sanitized tail to isolated 0600 file

## Source
Lines 62–78 in `crates/oxide-security/src/stderr_sanitizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stderr_sanitizer](/crates/oxide-security/src/stderr_sanitizer.md) |
