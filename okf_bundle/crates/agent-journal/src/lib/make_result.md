---
okf_version: "0.2"
type: Function
title: make_result
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib/make_result
language: rust
---

# make_result

## Signature

```rust
fn make_result(summary: &str) -> TaskResult
```

## Source
Lines 319–328 in `crates/agent-journal/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/agent-journal/src/lib.md) |
| called_by | [task_result_serializes_cleanly](/crates/agent-journal/src/lib/task_result_serializes_cleanly.md) |
