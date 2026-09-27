# budget

## Classs

- [BudgetConfig](BudgetConfig.md) — Budget configuration and trackers for token and tool execution governance.
- [BudgetTracker](BudgetTracker.md) — Dynamic tracker for token and tool call consumption.
- [BudgetViolation](BudgetViolation.md) — [derive(Debug, Clone, PartialEq, Eq)]

## Functions

- [default](default.md)
- [default](default_1.md)
- [estimated_cost_dollars](estimated_cost_dollars.md) — Calculate estimated dollar cost based on model blend pricing ($0.20 per 1M local tokens amortized)
- [estimated_cost_dollars](estimated_cost_dollars_1.md) — Calculate estimated dollar cost based on model blend pricing ($0.20 per 1M local tokens amortized)
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [get_session_tokens_used](get_session_tokens_used.md)
- [get_session_tokens_used](get_session_tokens_used_1.md)
- [get_task_tokens_used](get_task_tokens_used.md)
- [get_task_tokens_used](get_task_tokens_used_1.md)
- [get_task_tool_calls](get_task_tool_calls.md)
- [get_task_tool_calls](get_task_tool_calls_1.md)
- [new](new.md)
- [new](new_1.md)
- [record_tokens](record_tokens.md) — Record tokens used and check budget thresholds.
- [record_tokens](record_tokens_1.md) — Record tokens used and check budget thresholds.
- [record_tool_call](record_tool_call.md) — Record a tool invocation and success/failure status.
- [record_tool_call](record_tool_call_1.md) — Record a tool invocation and success/failure status.
- [reset_task](reset_task.md) — Reset per-task counters when starting a new task node.
- [reset_task](reset_task_1.md) — Reset per-task counters when starting a new task node.
- [test_budget_token_limits](test_budget_token_limits.md) — [test]
- [test_budget_tool_call_failures](test_budget_tool_call_failures.md) — [test]
