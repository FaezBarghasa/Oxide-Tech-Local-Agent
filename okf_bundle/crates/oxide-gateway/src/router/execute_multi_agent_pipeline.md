---
okf_version: "0.2"
type: Function
title: execute_multi_agent_pipeline
description: "Execute a multi-stage task through the multi-agent coordinator:"
resource: crates/oxide-gateway/src/router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:13:49Z"
concept_id: crates/oxide-gateway/src/router/execute_multi_agent_pipeline
language: rust
---

# execute_multi_agent_pipeline

Execute a multi-stage task through the multi-agent coordinator:

## Signature

```rust
impl GatewayRouter { pub fn execute_multi_agent_pipeline(
        &self,
        supervisor_id: &str,
        worker_ids: &[&str],
        verifier_id: &str,
        task: &str,
    ) -> Result<(String, String), anyhow::Error> }
```

## Visibility

- `pub`

## Docstring

Execute a multi-stage task through the multi-agent coordinator:
Supervisor plans -> Worker(s) execute in parallel -> Verifier synthesises.

## Source
Lines 174–184 in `crates/oxide-gateway/src/router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [router](/crates/oxide-gateway/src/router.md) |
