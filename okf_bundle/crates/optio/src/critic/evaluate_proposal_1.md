---
okf_version: "0.2"
type: Function
title: evaluate_proposal
description: Evaluate an implementation against task specification and constraints.
resource: crates/optio/src/critic.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T11:45:13Z"
concept_id: crates/optio/src/critic/evaluate_proposal_1
language: rust
---

# evaluate_proposal

Evaluate an implementation against task specification and constraints.

## Signature

```rust
pub fn evaluate_proposal(
        &self,
        task_desc: &str,
        proposed_action: &str,
        system_constraints: &[&str],
    ) -> Result<Critique>
```

## Visibility

- `pub`

## Docstring

Evaluate an implementation against task specification and constraints.

## Source
Lines 32–101 in `crates/optio/src/critic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [critic](/crates/optio/src/critic.md) |
