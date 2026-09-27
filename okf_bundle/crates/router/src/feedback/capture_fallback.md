---
okf_version: "0.2"
type: Function
title: capture_fallback
resource: crates/router/src/feedback.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/feedback/capture_fallback
language: rust
---

# capture_fallback

## Signature

```rust
impl CloudResponseCapture { pub fn capture_fallback(
        prompt: String,
        context: ContextSnapshot,
        cloud_response: CloudResponse,
        validation: ValidationResult,
    ) -> Result<(), anyhow::Error> }
```

## Visibility

- `pub`

## Source
Lines 8–31 in `crates/router/src/feedback.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [feedback](/crates/router/src/feedback.md) |
