---
okf_version: "0.2"
type: Module
title: lib
description: "# Agent Journal"
resource: crates/agent-journal/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:agent-journal"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/agent-journal/src/lib
language: rust
---

# lib

# Agent Journal

## Docstring

# Agent Journal

Durable, event-sourced journal for the multi-agent DAG executor.

Every `TaskNode` state transition is **appended** to the `agent_journal` SurrealDB
table — never mutated in place. On restart the supervisor replays the journal to
restore the exact `TaskDag` state at the last committed boundary, giving crash-safe
durable execution comparable to LangGraph's `SqliteSaver` / Restate journal.

## Relationships

| Type | Target |
|------|--------|
| related | [JournalError](/crates/agent-journal/src/lib/JournalError.md) |
| related | [ArtifactRef](/crates/agent-journal/src/lib/ArtifactRef.md) |
| related | [TaskResult](/crates/agent-journal/src/lib/TaskResult.md) |
| related | [HitlDecision](/crates/agent-journal/src/lib/HitlDecision.md) |
| related | [JournalEvent](/crates/agent-journal/src/lib/JournalEvent.md) |
| related | [JournalEntry](/crates/agent-journal/src/lib/JournalEntry.md) |
| related | [AgentJournal](/crates/agent-journal/src/lib/AgentJournal.md) |
| related | [new](/crates/agent-journal/src/lib/new.md) |
| related | [append](/crates/agent-journal/src/lib/append.md) |
| related | [replay_dag](/crates/agent-journal/src/lib/replay_dag.md) |
| related | [tail](/crates/agent-journal/src/lib/tail.md) |
| related | [purge_dag](/crates/agent-journal/src/lib/purge_dag.md) |
| related | [new](/crates/agent-journal/src/lib/new.md) |
| related | [append](/crates/agent-journal/src/lib/append.md) |
| related | [replay_dag](/crates/agent-journal/src/lib/replay_dag.md) |
| related | [tail](/crates/agent-journal/src/lib/tail.md) |
| related | [purge_dag](/crates/agent-journal/src/lib/purge_dag.md) |
| related | [ReplayedDagState](/crates/agent-journal/src/lib/ReplayedDagState.md) |
| related | [from_entries](/crates/agent-journal/src/lib/from_entries.md) |
| related | [from_entries](/crates/agent-journal/src/lib/from_entries.md) |
| related | [make_result](/crates/agent-journal/src/lib/make_result.md) |
| related | [replay_basic_dag_lifecycle](/crates/agent-journal/src/lib/replay_basic_dag_lifecycle.md) |
| related | [task_result_serializes_cleanly](/crates/agent-journal/src/lib/task_result_serializes_cleanly.md) |
| related | [journal_event_tagged_serialization](/crates/agent-journal/src/lib/journal_event_tagged_serialization.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [surrealdb](/_dependencies/cargo/surrealdb.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
