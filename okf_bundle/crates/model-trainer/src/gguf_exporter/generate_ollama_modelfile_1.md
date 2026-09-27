---
okf_version: "0.2"
type: Function
title: generate_ollama_modelfile
description: "Generate a production Ollama `Modelfile` linking directly to the exported GGUF file."
resource: crates/model-trainer/src/gguf_exporter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/model-trainer/src/gguf_exporter/generate_ollama_modelfile_1
language: rust
---

# generate_ollama_modelfile

Generate a production Ollama `Modelfile` linking directly to the exported GGUF file.

## Signature

```rust
pub fn generate_ollama_modelfile(
        modelfile_path: &Path,
        gguf_relative_path: &str,
        system_prompt: Option<&str>,
        temperature: f32,
    ) -> Result<(), TrainerError>
```

## Visibility

- `pub`

## Docstring

Generate a production Ollama `Modelfile` linking directly to the exported GGUF file.

## Source
Lines 73–112 in `crates/model-trainer/src/gguf_exporter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [gguf_exporter](/crates/model-trainer/src/gguf_exporter.md) |
