---
okf_version: "0.2"
type: Function
title: query_sglang_models
description: Query local SGLang / vLLM API for served models
resource: src-tauri/src/model_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: src-tauri/src/model_ipc/query_sglang_models
language: rust
---

# query_sglang_models

Query local SGLang / vLLM API for served models

## Signature

```rust
fn query_sglang_models(client: &reqwest::Client) -> Vec<ModelInfo>
```

## Docstring

Query local SGLang / vLLM API for served models

## Source
Lines 149–178 in `src-tauri/src/model_ipc.rs`
