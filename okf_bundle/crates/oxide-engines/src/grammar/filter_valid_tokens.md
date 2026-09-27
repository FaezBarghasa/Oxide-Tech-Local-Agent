---
okf_version: "0.2"
type: Function
title: filter_valid_tokens
description: Filter valid next token candidates given the current prefix state
resource: crates/oxide-engines/src/grammar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:09:50Z"
concept_id: crates/oxide-engines/src/grammar/filter_valid_tokens
language: rust
---

# filter_valid_tokens

Filter valid next token candidates given the current prefix state

## Signature

```rust
impl GbnfCompiler { pub fn filter_valid_tokens(
        &self,
        current_text: &str,
        vocabulary: &[String],
    ) -> HashSet<usize> }
```

## Visibility

- `pub`

## Docstring

Filter valid next token candidates given the current prefix state

## Source
Lines 83–103 in `crates/oxide-engines/src/grammar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grammar](/crates/oxide-engines/src/grammar.md) |
