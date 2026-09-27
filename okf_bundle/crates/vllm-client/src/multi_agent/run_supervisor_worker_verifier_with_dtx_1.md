---
okf_version: "0.2"
type: Function
title: run_supervisor_worker_verifier_with_dtx
description: "**Supervisor–Worker–Verifier with DTX**:"
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/run_supervisor_worker_verifier_with_dtx_1
language: rust
---

# run_supervisor_worker_verifier_with_dtx

**Supervisor–Worker–Verifier with DTX**:

## Signature

```rust
pub fn run_supervisor_worker_verifier_with_dtx(
        &self,
        supervisor_id: &str,
        worker_ids: &[&str],
        verifier_id: &str,
        task: &str,
        dtx_id: Option<String>,
    ) -> Result<(String, String)>
```

## Visibility

- `pub`

## Docstring

**Supervisor–Worker–Verifier with DTX**:
Full SWV pipeline propagating a single distributed audit DTX token across all steps.

## Source
Lines 534–697 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
