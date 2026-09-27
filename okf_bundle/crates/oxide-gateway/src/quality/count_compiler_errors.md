---
okf_version: "0.2"
type: Function
title: count_compiler_errors
description: "Count `error[E...]` and bare `error:` lines in Cargo stderr."
resource: crates/oxide-gateway/src/quality.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: crates/oxide-gateway/src/quality/count_compiler_errors
language: rust
---

# count_compiler_errors

Count `error[E...]` and bare `error:` lines in Cargo stderr.

## Signature

```rust
fn count_compiler_errors(stderr: &str) -> usize
```

## Docstring

Count `error[E...]` and bare `error:` lines in Cargo stderr.

Cargo emits errors in two forms:
- `error[E0308]: ...`  (typed compiler errors)
- `error: ...`         (linker / proc-macro errors)

We count both conservatively to avoid false positives from warnings
that start with "warning: unused ...".

## Source
Lines 62–72 in `crates/oxide-gateway/src/quality.rs`
