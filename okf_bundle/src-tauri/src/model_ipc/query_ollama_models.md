---
okf_version: "0.2"
type: Function
title: query_ollama_models
description: Query local Ollama API for installed models
resource: src-tauri/src/model_ipc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: src-tauri/src/model_ipc/query_ollama_models
language: rust
---

# query_ollama_models

Query local Ollama API for installed models

## Signature

```rust
fn query_ollama_models(client: &reqwest::Client) -> Vec<ModelInfo>
```

## Docstring

Query local Ollama API for installed models

## Source
Lines 109–146 in `src-tauri/src/model_ipc.rs`
