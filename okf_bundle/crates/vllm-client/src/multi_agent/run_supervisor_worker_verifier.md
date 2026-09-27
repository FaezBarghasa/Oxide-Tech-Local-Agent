---
okf_version: "0.2"
type: Function
title: run_supervisor_worker_verifier
description: "**Supervisor–Worker–Verifier** pattern:"
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/run_supervisor_worker_verifier
language: rust
---

# run_supervisor_worker_verifier

**Supervisor–Worker–Verifier** pattern:

## Signature

```rust
impl MultiAgentCoordinator { pub fn run_supervisor_worker_verifier(
        &self,
        supervisor_id: &str,
        worker_ids: &[&str],
        verifier_id: &str,
        task: &str,
    ) -> Result<(String, String)> }
```

## Visibility

- `pub`

## Docstring

**Supervisor–Worker–Verifier** pattern:

1. Supervisor receives the task and produces a work plan.
2. Each worker receives the work plan and produces output.
3. Verifier scores all worker outputs and picks the best (or requests revisions).

Returns the verifier's final synthesis and the thread ID.

## Source
Lines 515–530 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
