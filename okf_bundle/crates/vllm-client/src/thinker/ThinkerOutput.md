---
okf_version: "0.2"
type: Class
title: ThinkerOutput
description: The structured output produced by the Thinker model.
resource: crates/vllm-client/src/thinker.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/vllm-client/src/thinker/ThinkerOutput
language: rust
---

# ThinkerOutput

The structured output produced by the Thinker model.

## Signature

```rust
pub struct ThinkerOutput
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

The structured output produced by the Thinker model.

The Thinker never writes code.  It reasons about architecture, selects
design patterns, identifies which files to touch, and then distils all
of that reasoning into a single `coder_prompt` that the downstream coder
model can act on.
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `chosen_pattern`
- `architecture_notes`
- `files_to_touch`
- `coder_prompt`

## Source
Lines 17–28 in `crates/vllm-client/src/thinker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thinker](/crates/vllm-client/src/thinker.md) |
