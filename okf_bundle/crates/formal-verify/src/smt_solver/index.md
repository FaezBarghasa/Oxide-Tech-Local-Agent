# smt_solver

## Classs

- [CircuitState](CircuitState.md) — Circuit electrical state for SMT-LIB2 invariant translation:
- [SmtCircuitSafetyProof](SmtCircuitSafetyProof.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [TaskConstraint](TaskConstraint.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [VerifiedSchedule](VerifiedSchedule.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [VerifyError](VerifyError.md) — [derive(Error, Debug)]

## Functions

- [solve_task_schedule](solve_task_schedule.md) — SMT constraint formulation and deterministic EDF solver with mathematical proof of schedulability.
- [verify_circuit_safety_invariants](verify_circuit_safety_invariants.md) — Translate circuit state machine into SMT-LIB2 assertions and solve safety invariants
