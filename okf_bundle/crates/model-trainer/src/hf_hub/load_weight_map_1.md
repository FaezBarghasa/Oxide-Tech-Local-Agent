---
okf_version: "0.2"
type: Function
title: load_weight_map
description: Load the sharded weight index or single safetensors file map
resource: crates/model-trainer/src/hf_hub.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:06:47Z"
concept_id: crates/model-trainer/src/hf_hub/load_weight_map_1
language: rust
---

# load_weight_map

Load the sharded weight index or single safetensors file map

## Signature

```rust
pub fn load_weight_map(model_dir: &Path) -> Result<HashMap<String, PathBuf>, HfHubError>
```

## Visibility

- `pub`

## Docstring

Load the sharded weight index or single safetensors file map

## Source
Lines 73–108 in `crates/model-trainer/src/hf_hub.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hf_hub](/crates/model-trainer/src/hf_hub.md) |
