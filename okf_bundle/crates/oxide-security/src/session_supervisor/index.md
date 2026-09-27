# session_supervisor

## Classs

- [SessionReceipt](SessionReceipt.md) — Fsynced Receipt persisted prior to acknowledging turns
- [SessionState](SessionState.md) — Idempotent Agent Session Lifecycle State
- [SessionSupervisor](SessionSupervisor.md) — Process Supervisor managing idempotent sessions and single-attempt crash recovery
- [SessionSupervisorError](SessionSupervisorError.md) — [derive(Debug, Error)]

## Functions

- [admit_or_resume_session](admit_or_resume_session.md) — Obtain or create an idempotent session receipt
- [admit_or_resume_session](admit_or_resume_session_1.md) — Obtain or create an idempotent session receipt
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [persist_receipt_sync](persist_receipt_sync.md) — Fsync receipt to disk
- [persist_receipt_sync](persist_receipt_sync_1.md) — Fsync receipt to disk
- [test_idempotent_session_creation_and_recovery_limit](test_idempotent_session_creation_and_recovery_limit.md) — [tokio::test]
- [transition_state](transition_state.md) — Transition session state and enforce single crash recovery attempt
- [transition_state](transition_state_1.md) — Transition session state and enforce single crash recovery attempt
