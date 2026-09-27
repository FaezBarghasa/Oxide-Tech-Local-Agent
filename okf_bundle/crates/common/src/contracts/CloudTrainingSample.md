---
okf_version: "0.2"
type: Class
title: CloudTrainingSample
description: "[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]"
resource: crates/common/src/contracts.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:common"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/common/src/contracts/CloudTrainingSample
language: rust
---

# CloudTrainingSample

[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]

## Signature

```rust
pub struct CloudTrainingSample
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]

## Methods

- `id`
- `tenant_id`
- `user_id`
- `task_type`
- `prompt`
- `cloud_provider`
- `cloud_response`
- `cloud_reasoning`
- `validation`
- `confidence`
- `cost_usd`
- `user_outcome`

## Source
Lines 69–82 in `crates/common/src/contracts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [contracts](/crates/common/src/contracts.md) |
