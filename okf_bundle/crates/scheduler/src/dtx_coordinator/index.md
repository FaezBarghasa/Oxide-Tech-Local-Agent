# dtx_coordinator

## Classs

- [DtxCoordinator](DtxCoordinator.md) — [derive(Clone, Default)]
- [MockTransactionalResource](MockTransactionalResource.md)

## Functions

- [abort_saga](abort_saga.md)
- [abort_saga](abort_saga_1.md)
- [begin_dtx](begin_dtx.md) — Begin a new distributed transaction across multiple domains
- [begin_dtx](begin_dtx_1.md) — Begin a new distributed transaction across multiple domains
- [commit](commit.md)
- [commit](commit_1.md)
- [commit_dtx](commit_dtx.md) — Mark a distributed transaction as committed
- [commit_dtx](commit_dtx_1.md) — Mark a distributed transaction as committed
- [execute_saga](execute_saga.md) — Execute a full Saga transaction with pre-flight irreversible check, compensation snapshotting,
- [execute_saga](execute_saga_1.md) — Execute a full Saga transaction with pre-flight irreversible check, compensation snapshotting,
- [fmt](fmt.md)
- [fmt](fmt_1.md)
- [get_status](get_status.md) — Query the status of a distributed transaction
- [get_status](get_status_1.md) — Query the status of a distributed transaction
- [is_irreversible](is_irreversible.md)
- [is_irreversible](is_irreversible_1.md)
- [new](new.md)
- [new](new_1.md)
- [prepare](prepare.md)
- [prepare](prepare_1.md)
- [register_resource](register_resource.md) — Register a participating transactional resource
- [register_resource](register_resource_1.md) — Register a participating transactional resource
- [rollback](rollback.md)
- [rollback](rollback_1.md)
- [rollback_dtx](rollback_dtx.md) — Broadcast rollback across all domain apps for a failed multi-domain task
- [rollback_dtx](rollback_dtx_1.md) — Broadcast rollback across all domain apps for a failed multi-domain task
- [snapshot_for_compensation](snapshot_for_compensation.md)
- [snapshot_for_compensation](snapshot_for_compensation_1.md)
- [test_dtx_saga_commit_and_abort](test_dtx_saga_commit_and_abort.md) — [tokio::test]
