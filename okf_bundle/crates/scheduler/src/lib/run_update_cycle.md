---
okf_version: "0.2"
type: Function
title: run_update_cycle
resource: crates/scheduler/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T09:51:31Z"
concept_id: crates/scheduler/src/lib/run_update_cycle
language: rust
---

# run_update_cycle

## Signature

```rust
pub fn run_update_cycle(config: &AppConfig, db: &Surreal<Any>) -> Result<(), anyhow::Error>
```

## Visibility

- `pub`

## Source
Lines 91–206 in `crates/scheduler/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/scheduler/src/lib.md) |
| calls | [parse_feed](/crates/knowledge/src/rss/parse_feed.md) |
| calls | [curate_and_publish_news](/crates/scheduler/src/lib/curate_and_publish_news.md) |
| called_by | [start_scheduler](/crates/scheduler/src/lib/start_scheduler.md) |
