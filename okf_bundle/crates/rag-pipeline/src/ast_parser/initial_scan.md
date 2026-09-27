---
okf_version: "0.2"
type: Function
title: initial_scan
resource: crates/rag-pipeline/src/ast_parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:rag-pipeline"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-07-25T15:09:12Z"
concept_id: crates/rag-pipeline/src/ast_parser/initial_scan
language: rust
---

# initial_scan

## Signature

```rust
impl AstParser { fn initial_scan(root: &PathBuf, symbols: &Arc<Mutex<HashMap<PathBuf, Vec<String>>>>) -> NotifyResult<()> }
```

## Source
Lines 60–81 in `crates/rag-pipeline/src/ast_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ast_parser](/crates/rag-pipeline/src/ast_parser.md) |
