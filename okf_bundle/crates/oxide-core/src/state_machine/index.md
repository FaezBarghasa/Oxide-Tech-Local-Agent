# state_machine

## Classs

- [AgentState](AgentState.md) — Comprehensive state enum for the agent loop lifecycle.
- [AgentStateMachine](AgentStateMachine.md) — Pinned, observable agent state machine driver.
- [StateTransition](StateTransition.md) — Recorded state transition with timestamp.

## Functions

- [current](current.md) — Read the current state directly.
- [current](current_1.md) — Read the current state directly.
- [default](default.md)
- [default](default_1.md)
- [fault](fault.md) — Transition directly to Fault state from any active state.
- [fault](fault_1.md) — Transition directly to Fault state from any active state.
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [new](new.md) — Create a new state machine initialized in `Idle`.
- [new](new_1.md) — Create a new state machine initialized in `Idle`.
- [subscribe](subscribe.md) — Obtain a lock-free watch receiver for downstream state subscribers.
- [subscribe](subscribe_1.md) — Obtain a lock-free watch receiver for downstream state subscribers.
- [test_fault_isolation](test_fault_isolation.md) — [test]
- [test_illegal_transition_rejection](test_illegal_transition_rejection.md) — [test]
- [test_valid_lifecycle_transitions](test_valid_lifecycle_transitions.md) — [test]
- [total_transitions](total_transitions.md) — Total valid transitions executed.
- [total_transitions](total_transitions_1.md) — Total valid transitions executed.
- [transition_to](transition_to.md) — Attempt a deterministic state transition, validating legal state paths.
- [transition_to](transition_to_1.md) — Attempt a deterministic state transition, validating legal state paths.
- [validate_transition](validate_transition.md) — Enforce valid state machine transitions according to deterministic lifecycle rules.
- [validate_transition](validate_transition_1.md) — Enforce valid state machine transitions according to deterministic lifecycle rules.
