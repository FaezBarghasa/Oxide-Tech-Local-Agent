---
okf_version: "0.2"
type: Function
title: complete_prompt
description: Run a generic LLM completion query using the configured thinker model.
resource: crates/thinker/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:thinker"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/thinker/src/lib/complete_prompt_1
language: rust
---

# complete_prompt

Run a generic LLM completion query using the configured thinker model.

## Signature

```rust
pub fn complete_prompt(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        config: &AppConfig,
    ) -> Result<String, anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Run a generic LLM completion query using the configured thinker model.

## Source
Lines 58–65 in `crates/thinker/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/thinker/src/lib.md) |
| calls | [complete_llm](/crates/thinker/src/lib/complete_llm.md) |
