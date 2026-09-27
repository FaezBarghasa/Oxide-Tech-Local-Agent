---
okf_version: "0.2"
type: Function
title: generate_mojo_spec_from_llm
resource: crates/self-evolver/src/tool_maker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/self-evolver/src/tool_maker/generate_mojo_spec_from_llm_1
language: rust
---

# generate_mojo_spec_from_llm

## Signature

```rust
pub fn generate_mojo_spec_from_llm(
        &self,
        system: &str,
        prompt: &str,
    ) -> Result<MojoToolSpecification>
```

## Visibility

- `pub`

## Source
Lines 223–252 in `crates/self-evolver/src/tool_maker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tool_maker](/crates/self-evolver/src/tool_maker.md) |
