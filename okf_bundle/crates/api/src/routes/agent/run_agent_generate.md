---
okf_version: "0.2"
type: Function
title: run_agent_generate
description: ── Core generation logic ─────────────────────────────────────────────────────
resource: crates/api/src/routes/agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:18:37Z"
concept_id: crates/api/src/routes/agent/run_agent_generate
language: rust
---

# run_agent_generate

── Core generation logic ─────────────────────────────────────────────────────

## Signature

```rust
pub fn run_agent_generate(
    req: GenerateRequest,
    app_cfg: &AppConfig,
    rag_pipeline: Option<&RagPipeline>,
) -> Result<serde_json::Value, String>
```

## Visibility

- `pub`

## Docstring

── Core generation logic ─────────────────────────────────────────────────────

## Source
Lines 154–334 in `crates/api/src/routes/agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent](/crates/api/src/routes/agent.md) |
| calls | [collect_rs_files](/crates/api/src/routes/agent/collect_rs_files.md) |
| calls | [strip_json_fences](/crates/api/src/routes/agent/strip_json_fences.md) |
| calls | [write_file_async](/crates/api/src/routes/agent/write_file_async.md) |
| calls | [apply_diff_async](/crates/api/src/routes/agent/apply_diff_async.md) |
| called_by | [handle_agent_generate](/crates/api/src/routes/agent/handle_agent_generate.md) |
