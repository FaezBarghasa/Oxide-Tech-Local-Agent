---
okf_version: "0.2"
type: Function
title: verify_in_sandbox
description: Run sandboxed cargo-check or test command and format standard feedback for the Verifier agent.
resource: crates/verifier/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/lib/verify_in_sandbox
language: rust
---

# verify_in_sandbox

Run sandboxed cargo-check or test command and format standard feedback for the Verifier agent.

## Signature

```rust
impl MultiAgentVerifierBridge { pub fn verify_in_sandbox(&self, cmd: &[&str], work_dir: &str) -> Result<VerifierReport> }
```

## Visibility

- `pub`

## Docstring

Run sandboxed cargo-check or test command and format standard feedback for the Verifier agent.

## Source
Lines 146–158 in `crates/verifier/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/verifier/src/lib.md) |
| calls | [execute_in_sandbox](/crates/verifier/src/lib/execute_in_sandbox.md) |
