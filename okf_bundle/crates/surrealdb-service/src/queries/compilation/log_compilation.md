---
okf_version: "0.2"
type: Function
title: log_compilation
resource: crates/surrealdb-service/src/queries/compilation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:surrealdb-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/surrealdb-service/src/queries/compilation/log_compilation
language: rust
---

# log_compilation

## Signature

```rust
pub fn log_compilation(
    client: &SurrealClient,
    project_id: RecordId,
    success: bool,
    stdout: &str,
    stderr: &str,
) -> Result<CompilationRecord, Error>
```

## Visibility

- `pub`

## Source
Lines 6–26 in `crates/surrealdb-service/src/queries/compilation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [compilation](/crates/surrealdb-service/src/queries/compilation.md) |
