---
okf_version: "0.2"
type: Function
title: with_journal
description: Attach a durable journal to this supervisor for crash-safe execution.
resource: crates/router/src/supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/supervisor/with_journal
language: rust
---

# with_journal

Attach a durable journal to this supervisor for crash-safe execution.

## Signature

```rust
impl SupervisorAgent { pub fn with_journal(mut self, journal: AgentJournal) -> Self }
```

## Visibility

- `pub`

## Docstring

Attach a durable journal to this supervisor for crash-safe execution.

## Source
Lines 234–237 in `crates/router/src/supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supervisor](/crates/router/src/supervisor.md) |
