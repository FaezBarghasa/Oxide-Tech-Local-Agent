---
okf_version: "0.2"
type: Function
title: export_merged_gguf
description: Export merged model and adapter into a self-contained GGUF v3 binary.
resource: crates/model-trainer/src/gguf_exporter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/model-trainer/src/gguf_exporter/export_merged_gguf_1
language: rust
---

# export_merged_gguf

Export merged model and adapter into a self-contained GGUF v3 binary.

## Signature

```rust
pub fn export_merged_gguf(
        output_path: &Path,
        model_name: &str,
        quant_type: GgufQuantType,
        context_length: usize,
    ) -> Result<PathBuf, TrainerError>
```

## Visibility

- `pub`

## Docstring

Export merged model and adapter into a self-contained GGUF v3 binary.

## Source
Lines 10–70 in `crates/model-trainer/src/gguf_exporter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gguf_exporter](/crates/model-trainer/src/gguf_exporter.md) |
