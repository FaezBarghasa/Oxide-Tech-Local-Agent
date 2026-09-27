---
okf_version: "0.2"
type: Function
title: mask_logits
description: Apply hard logit mask to invalid token indices
resource: crates/oxide-engines/src/grammar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:09:50Z"
concept_id: crates/oxide-engines/src/grammar/mask_logits
language: rust
---

# mask_logits

Apply hard logit mask to invalid token indices

## Signature

```rust
impl GbnfCompiler { pub fn mask_logits(&self, logits: &mut [f32], valid_indices: &HashSet<usize>) }
```

## Visibility

- `pub`

## Docstring

Apply hard logit mask to invalid token indices

## Source
Lines 135–141 in `crates/oxide-engines/src/grammar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grammar](/crates/oxide-engines/src/grammar.md) |
