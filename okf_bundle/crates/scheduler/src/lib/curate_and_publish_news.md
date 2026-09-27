---
okf_version: "0.2"
type: Function
title: curate_and_publish_news
resource: crates/scheduler/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T09:51:31Z"
concept_id: crates/scheduler/src/lib/curate_and_publish_news
language: rust
---

# curate_and_publish_news

## Signature

```rust
fn curate_and_publish_news(
    config: &AppConfig,
    db: &Surreal<Any>,
    thinker_client: &ThinkerClient,
    items: Vec<RawNewsItem>,
) -> Result<(), anyhow::Error>
```

## Source
Lines 208–271 in `crates/scheduler/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/scheduler/src/lib.md) |
| calls | [create_post](/crates/blog/src/service/create_post.md) |
| called_by | [run_update_cycle](/crates/scheduler/src/lib/run_update_cycle.md) |
