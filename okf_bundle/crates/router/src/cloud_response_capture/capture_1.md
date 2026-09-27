---
okf_version: "0.2"
type: Function
title: capture
resource: crates/router/src/cloud_response_capture.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:05:01Z"
concept_id: crates/router/src/cloud_response_capture/capture_1
language: rust
---

# capture

## Signature

```rust
pub fn capture(
        &self,
        tenant_id: String,
        user_id: String,
        task_type: String,
        prompt: String,
        cloud_provider: String,
        cloud_response: String,
        cloud_reasoning: Option<String>,
        confidence: f32,
        cost_usd: f32,
    ) -> Result<Uuid, anyhow::Error>
```

## Visibility

- `pub`

## Source
Lines 16–61 in `crates/router/src/cloud_response_capture.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cloud_response_capture](/crates/router/src/cloud_response_capture.md) |
