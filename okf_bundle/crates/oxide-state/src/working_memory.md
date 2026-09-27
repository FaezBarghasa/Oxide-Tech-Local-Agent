---
okf_version: "0.2"
type: Module
title: working_memory
description: "# Working Memory"
resource: crates/oxide-state/src/working_memory.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/working_memory
language: rust
---

# working_memory

# Working Memory

## Docstring

# Working Memory

Per-agent and per-session scoped memory buffers.
Isolates transient scratchpads, sub-agent observations, and partial artifacts
so concurrent sub-agents do not cross-contaminate each other's context windows.

## Relationships

| Type | Target |
|------|--------|
| related | [WorkingMemoryEntry](/crates/oxide-state/src/working_memory/WorkingMemoryEntry.md) |
| related | [WorkingMemoryManager](/crates/oxide-state/src/working_memory/WorkingMemoryManager.md) |
| related | [new](/crates/oxide-state/src/working_memory/new.md) |
| related | [set](/crates/oxide-state/src/working_memory/set.md) |
| related | [get](/crates/oxide-state/src/working_memory/get.md) |
| related | [get_all_for_agent](/crates/oxide-state/src/working_memory/get_all_for_agent.md) |
| related | [clear_session](/crates/oxide-state/src/working_memory/clear_session.md) |
| related | [consolidate_and_compress](/crates/oxide-state/src/working_memory/consolidate_and_compress.md) |
| related | [new](/crates/oxide-state/src/working_memory/new.md) |
| related | [set](/crates/oxide-state/src/working_memory/set.md) |
| related | [get](/crates/oxide-state/src/working_memory/get.md) |
| related | [get_all_for_agent](/crates/oxide-state/src/working_memory/get_all_for_agent.md) |
| related | [clear_session](/crates/oxide-state/src/working_memory/clear_session.md) |
| related | [consolidate_and_compress](/crates/oxide-state/src/working_memory/consolidate_and_compress.md) |
| related | [chrono](/_dependencies/cargo/chrono.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [surrealdb](/_dependencies/cargo/surrealdb.md) |
