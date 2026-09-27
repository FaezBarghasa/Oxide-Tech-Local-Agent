---
okf_version: "0.2"
type: Function
title: new
description: Create a new McpServer instance.
resource: crates/oxide-mcp/src/handler.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-mcp"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:25:35Z"
concept_id: crates/oxide-mcp/src/handler/new
language: rust
---

# new

Create a new McpServer instance.

## Signature

```rust
impl McpServer { pub fn new(
        workspace_root: PathBuf,
        rag: Option<Arc<rag_pipeline::RagPipeline>>,
        inference_provider: Option<Arc<dyn vllm_client::InferenceProvider>>,
    ) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a new McpServer instance.

## Source
Lines 153–164 in `crates/oxide-mcp/src/handler.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [handler](/crates/oxide-mcp/src/handler.md) |
