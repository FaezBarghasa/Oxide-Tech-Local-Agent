# supervisor

## Classs

- [HitlHandle](HitlHandle.md) — A pending HITL approval that can suspend and resume a task.
- [SubAgentRole](SubAgentRole.md) — Specialized Sub-Agent roles in the multi-agent swarm.
- [SupervisorAgent](SupervisorAgent.md) — Multi-Agent Supervisor: plans, delegates, monitors, and guards against oscillation.
- [TaskDag](TaskDag.md) — Directed Acyclic Graph (DAG) for multi-agent coordination.
- [TaskNode](TaskNode.md) — A node in the execution DAG representing a delegated sub-agent task.
- [TaskStatus](TaskStatus.md) — Status of an individual task node in the execution DAG.

## Functions

- [add_task](add_task.md)
- [add_task](add_task_1.md)
- [dag_is_ready_respects_dependencies](dag_is_ready_respects_dependencies.md) — [test]
- [detect_oscillation](detect_oscillation.md) — Detect oscillation if the same error is seen N >= 3 times in a row.
- [detect_oscillation](detect_oscillation_1.md) — Detect oscillation if the same error is seen N >= 3 times in a row.
- [is_ready](is_ready.md) — Check if all dependencies for a node have passed.
- [is_ready](is_ready_1.md) — Check if all dependencies for a node have passed.
- [journal_append](journal_append.md) — Convenience: append a journal event if a journal is attached.
- [journal_append](journal_append_1.md) — Convenience: append a journal event if a journal is attached.
- [name](name.md)
- [name](name_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [parallel_execution_tiers](parallel_execution_tiers.md) — Group tasks that have no mutual dependencies — these can run in parallel.
- [parallel_execution_tiers](parallel_execution_tiers_1.md) — Group tasks that have no mutual dependencies — these can run in parallel.
- [parallel_tiers_single_chain](parallel_tiers_single_chain.md) — [test]
- [plan_goal](plan_goal.md) — Decomposes a user goal into a verified multi-agent task DAG and journals it.
- [plan_goal](plan_goal_1.md) — Decomposes a user goal into a verified multi-agent task DAG and journals it.
- [plan_goal_architect_mode_produces_one_task](plan_goal_architect_mode_produces_one_task.md) — [tokio::test]
- [plan_goal_code_mode_produces_parallel_tiers](plan_goal_code_mode_produces_parallel_tiers.md) — [tokio::test]
- [plan_goal_debug_mode_produces_diagnose_task](plan_goal_debug_mode_produces_diagnose_task.md) — [tokio::test]
- [system_prompt](system_prompt.md)
- [system_prompt](system_prompt_1.md)
- [with_journal](with_journal.md) — Attach a durable journal to this supervisor for crash-safe execution.
- [with_journal](with_journal_1.md) — Attach a durable journal to this supervisor for crash-safe execution.
