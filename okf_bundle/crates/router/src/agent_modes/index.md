# agent_modes

## Classs

- [AgentMode](AgentMode.md) — Operational modes that alter agent behavior, depth of reasoning, and tool permissions.
- [OpaqueConstructGuard](OpaqueConstructGuard.md) — Opaque Construct Guard (OpenWorker-Grade Shell Security)
- [PermissionDecision](PermissionDecision.md) — Granular Governance Decision
- [RiskClass](RiskClass.md) — 5-Tier Intrinsic Risk Classes (Aligned with OpenWorker Governance Architecture)
- [ToolPermissions](ToolPermissions.md) — Granular tool use permissions

## Functions

- [as_str](as_str.md)
- [as_str](as_str_1.md)
- [as_str](as_str_2.md)
- [as_str](as_str_3.md)
- [classify_tool_call](classify_tool_call.md) — Classify a tool call into its intrinsic Risk Class
- [evaluate_execution](evaluate_execution.md) — Comprehensive governance decision based on mode, tool risk, and arguments
- [evaluate_execution](evaluate_execution_1.md) — Comprehensive governance decision based on mode, tool risk, and arguments
- [for_mode](for_mode.md)
- [for_mode](for_mode_1.md)
- [from_str](from_str.md)
- [from_str](from_str_1.md)
- [has_dangerous_constructs](has_dangerous_constructs.md) — Check if a command includes dangerous execution/deletion flags or argument executors
- [has_dangerous_constructs](has_dangerous_constructs_1.md) — Check if a command includes dangerous execution/deletion flags or argument executors
- [has_opaque_constructs](has_opaque_constructs.md) — Check if a command string contains opaque expansions or unvetted redirection
- [has_opaque_constructs](has_opaque_constructs_1.md) — Check if a command string contains opaque expansions or unvetted redirection
- [is_consequential](is_consequential.md) — Returns true if the tool call carries consequential side effects
- [is_consequential](is_consequential_1.md) — Returns true if the tool call carries consequential side effects
- [strictness](strictness.md) — Strictness score for floor enforcement
- [strictness](strictness_1.md) — Strictness score for floor enforcement
- [system_prompt_directive](system_prompt_directive.md) — Returns the system prompt modifier customized for the operational mode.
- [system_prompt_directive](system_prompt_directive_1.md) — Returns the system prompt modifier customized for the operational mode.
- [test_opaque_construct_guard](test_opaque_construct_guard.md) — [test]
- [test_plan_mode_blocks_writes](test_plan_mode_blocks_writes.md) — [test]
- [test_review_mode_blocks_writes](test_review_mode_blocks_writes.md) — [test]
- [test_risk_classification](test_risk_classification.md) — [test]
- [test_unattended_parks_consequential](test_unattended_parks_consequential.md) — [test]
