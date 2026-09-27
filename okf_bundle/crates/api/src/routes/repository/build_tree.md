---
okf_version: "0.2"
type: Function
title: build_tree
resource: crates/api/src/routes/repository.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
concept_id: crates/api/src/routes/repository/build_tree
language: rust
---

# build_tree

## Signature

```rust
fn build_tree(dir: &Path, base_path: &Path) -> Option<Vec<FileNode>>
```

## Source
Lines 20–70 in `crates/api/src/routes/repository.rs`
