---
okf_version: "0.2"
type: Function
title: new
resource: crates/verifier/src/evidence.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/verifier/src/evidence/new
language: rust
---

# new

## Signature

```rust
impl EvidenceBundle { pub fn new(task_id: impl Into<String>, git_diff: impl Into<String>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 27–37 in `crates/verifier/src/evidence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [evidence](/crates/verifier/src/evidence.md) |
