---
okf_version: "0.2"
type: Function
title: run_repository_structure
resource: crates/api/src/routes/repository.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/repository/run_repository_structure
language: rust
---

# run_repository_structure

## Signature

```rust
pub fn run_repository_structure(workspace_path: String) -> Result<FileNode, String>
```

## Visibility

- `pub`

## Source
Lines 72–96 in `crates/api/src/routes/repository.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [repository](/crates/api/src/routes/repository.md) |
| calls | [build_tree](/crates/api/src/routes/repository/build_tree.md) |
| called_by | [handle_repository_structure](/crates/api/src/routes/repository/handle_repository_structure.md) |
