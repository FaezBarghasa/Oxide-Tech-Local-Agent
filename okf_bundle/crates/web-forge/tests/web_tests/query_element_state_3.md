---
okf_version: "0.2"
type: Function
title: query_element_state
resource: crates/web-forge/tests/web_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:21Z"
concept_id: crates/web-forge/tests/web_tests/query_element_state_3
language: rust
---

# query_element_state

## Signature

```rust
fn query_element_state(
        &self,
        selector: &str,
    ) -> Result<Option<ElementLayoutState>, WebForgeError>
```

## Source
Lines 36–52 in `crates/web-forge/tests/web_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [web_tests](/crates/web-forge/tests/web_tests.md) |
