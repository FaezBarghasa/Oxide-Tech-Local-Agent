---
okf_version: "0.2"
type: Function
title: complete_llm
resource: crates/thinker/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:thinker"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/thinker/src/lib/complete_llm
language: rust
---

# complete_llm

## Signature

```rust
fn complete_llm(
    config: &AppConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<String, anyhow::Error>
```

## Source
Lines 107–203 in `crates/thinker/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/thinker/src/lib.md) |
| called_by | [complete_prompt](/crates/thinker/src/lib/complete_prompt.md) |
| called_by | [plan_task](/crates/thinker/src/lib/plan_task.md) |
