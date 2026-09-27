---
okf_version: "0.2"
type: Function
title: analyze_and_prompt
description: Analyse the workspace context + user mandate + RAG chunks and produce a
resource: crates/vllm-client/src/thinker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/vllm-client/src/thinker/analyze_and_prompt_1
language: rust
---

# analyze_and_prompt

Analyse the workspace context + user mandate + RAG chunks and produce a

## Signature

```rust
pub fn analyze_and_prompt(
        &self,
        workspace_context: &str,
        user_mandate: &str,
        rag_context: &str,
        style_context: &str,
    ) -> Result<ThinkerOutput, anyhow::Error>
```

## Visibility

- `pub`

## Docstring

Analyse the workspace context + user mandate + RAG chunks and produce a
`ThinkerOutput` with a ready-to-use coder prompt.

`workspace_context` — the textual AST summary from SurrealDB.
`user_mandate`      — the raw user request.
`rag_context`       — top-k RAG chunks from the Qdrant pipeline.
`style_context`     — the user's coding style profile summary.

## Source
Lines 107–142 in `crates/vllm-client/src/thinker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thinker](/crates/vllm-client/src/thinker.md) |
| calls | [strip_json_fences](/crates/vllm-client/src/thinker/strip_json_fences.md) |
