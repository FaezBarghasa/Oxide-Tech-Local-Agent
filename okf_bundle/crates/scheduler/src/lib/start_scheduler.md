---
okf_version: "0.2"
type: Function
title: start_scheduler
resource: crates/scheduler/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T09:51:31Z"
concept_id: crates/scheduler/src/lib/start_scheduler
language: rust
---

# start_scheduler

## Signature

```rust
pub fn start_scheduler(config: AppConfig, db: Surreal<Any>) -> tokio::task::JoinHandle<()>
```

## Visibility

- `pub`

## Source
Lines 67–89 in `crates/scheduler/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/scheduler/src/lib.md) |
| calls | [run_update_cycle](/crates/scheduler/src/lib/run_update_cycle.md) |
