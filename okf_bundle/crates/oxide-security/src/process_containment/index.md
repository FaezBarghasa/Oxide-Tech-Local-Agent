# process_containment

## Classs

- [ProcessContainmentError](ProcessContainmentError.md) — [derive(Debug, Error)]
- [ProcessTreeGuard](ProcessTreeGuard.md) — Process Tree Guard tracking child processes and process groups to guarantee zero-zombie teardown

## Functions

- [new](new.md)
- [new](new_1.md)
- [pgid](pgid.md)
- [pgid](pgid_1.md)
- [pid](pid.md)
- [pid](pid_1.md)
- [terminate_all](terminate_all.md) — Clean teardown: sends SIGTERM to process group, waits up to grace_period, then forces SIGKILL
- [terminate_all](terminate_all_1.md) — Clean teardown: sends SIGTERM to process group, waits up to grace_period, then forces SIGKILL
- [test_process_tree_guard_init](test_process_tree_guard_init.md) — [tokio::test]
- [with_grace_period](with_grace_period.md)
- [with_grace_period](with_grace_period_1.md)
