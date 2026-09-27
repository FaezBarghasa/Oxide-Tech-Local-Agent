# lib

## Classs

- [AgentJournal](AgentJournal.md) — Thin wrapper around `Surreal<Any>` providing type-safe journal operations.
- [ArtifactRef](ArtifactRef.md) — Typed artifact reference — replaces the bare `Option<String>` result field.
- [HitlDecision](HitlDecision.md) — Human-in-the-loop decision recorded in the journal.
- [JournalEntry](JournalEntry.md) — A single immutable row in the `agent_journal` SurrealDB table.
- [JournalError](JournalError.md) — [derive(Debug, Error)]
- [JournalEvent](JournalEvent.md) — All possible journal events — the complete vocabulary of the agent's execution.
- [ReplayedDagState](ReplayedDagState.md) — Reconstructed DAG state after journal replay.
- [TaskResult](TaskResult.md) — Rich, typed result for a completed `TaskNode`.

## Functions

- [append](append.md) — Append a new journal event.
- [append](append_1.md) — Append a new journal event.
- [from_entries](from_entries.md) — Build reconstructed state from a sorted sequence of journal entries.
- [from_entries](from_entries_1.md) — Build reconstructed state from a sorted sequence of journal entries.
- [journal_event_tagged_serialization](journal_event_tagged_serialization.md) — [test]
- [make_result](make_result.md)
- [new](new.md) — Wrap an already-connected `Surreal` client.
- [new](new_1.md) — Wrap an already-connected `Surreal` client.
- [purge_dag](purge_dag.md) — Delete all journal entries for a completed DAG (optional cleanup after archival).
- [purge_dag](purge_dag_1.md) — Delete all journal entries for a completed DAG (optional cleanup after archival).
- [replay_basic_dag_lifecycle](replay_basic_dag_lifecycle.md) — [test]
- [replay_dag](replay_dag.md) — Replay all journal entries for a given `dag_id` in sequence order.
- [replay_dag](replay_dag_1.md) — Replay all journal entries for a given `dag_id` in sequence order.
- [tail](tail.md) — Return the last `N` journal entries across all DAGs (for live monitoring).
- [tail](tail_1.md) — Return the last `N` journal entries across all DAGs (for live monitoring).
- [task_result_serializes_cleanly](task_result_serializes_cleanly.md) — [test]
