---
okf_version: "0.2"
type: Function
title: run_chain_with_dtx
description: "**Sequential chain with DTX**: Executes run_chain with an explicit or generated DTX trace token."
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/run_chain_with_dtx_1
language: rust
---

# run_chain_with_dtx

**Sequential chain with DTX**: Executes run_chain with an explicit or generated DTX trace token.

## Signature

```rust
pub fn run_chain_with_dtx(
        &self,
        agent_ids: &[&str],
        initial_prompt: &str,
        dtx_id: Option<String>,
    ) -> Result<(String, String)>
```

## Visibility

- `pub`

## Docstring

**Sequential chain with DTX**: Executes run_chain with an explicit or generated DTX trace token.

## Source
Lines 466–506 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
