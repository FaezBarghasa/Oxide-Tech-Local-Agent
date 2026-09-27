---
okf_version: "0.2"
type: Function
title: plan_task
description: Analyse the task using the configured Thinker LLM and produce a detailed plan and coder prompt.
resource: crates/thinker/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:thinker"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/thinker/src/lib/plan_task
language: rust
---

# plan_task

Analyse the task using the configured Thinker LLM and produce a detailed plan and coder prompt.

## Signature

```rust
impl ThinkerClient { pub fn plan_task(
        &self,
        prompt: &str,
        config: &AppConfig,
    ) -> Result<ThinkerOutput, anyhow::Error> }
```

## Visibility

- `pub`

## Docstring

Analyse the task using the configured Thinker LLM and produce a detailed plan and coder prompt.

## Source
Lines 68–96 in `crates/thinker/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/thinker/src/lib.md) |
| calls | [complete_llm](/crates/thinker/src/lib/complete_llm.md) |
| calls | [clean_json_response](/crates/thinker/src/lib/clean_json_response.md) |
