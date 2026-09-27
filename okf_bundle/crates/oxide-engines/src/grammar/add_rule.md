---
okf_version: "0.2"
type: Function
title: add_rule
resource: crates/oxide-engines/src/grammar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:09:50Z"
concept_id: crates/oxide-engines/src/grammar/add_rule
language: rust
---

# add_rule

## Signature

```rust
impl GbnfCompiler { pub fn add_rule(&mut self, name: impl Into<String>, rule: GrammarRule) }
```

## Visibility

- `pub`

## Source
Lines 32–34 in `crates/oxide-engines/src/grammar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grammar](/crates/oxide-engines/src/grammar.md) |
