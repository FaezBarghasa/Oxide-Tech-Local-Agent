---
okf_version: "0.2"
type: Function
title: request_approval
description: Submit a high-risk action for Human-in-the-Loop approval and wait for confirmation.
resource: crates/scheduler/src/hitl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/hitl/request_approval
language: rust
---

# request_approval

Submit a high-risk action for Human-in-the-Loop approval and wait for confirmation.

## Signature

```rust
impl HitlApprovalChannel { pub fn request_approval(
        &self,
        req: HitlRequest,
        timeout_duration: Duration,
    ) -> Result<HitlDecision> }
```

## Visibility

- `pub`

## Docstring

Submit a high-risk action for Human-in-the-Loop approval and wait for confirmation.

## Source
Lines 55–89 in `crates/scheduler/src/hitl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hitl](/crates/scheduler/src/hitl.md) |
