# inbox

## Classs

- [HitlInboxManager](HitlInboxManager.md) — Manager coordinating in-memory parked channels and persisted inbox items.
- [InboxEntry](InboxEntry.md) — An inbox entry persisted to SurrealDB representing an action waiting for human review.
- [InboxStatus](InboxStatus.md) — State of an inbox item.

## Functions

- [list_pending](list_pending.md) — Retrieve all pending requests from the database.
- [list_pending](list_pending_1.md) — Retrieve all pending requests from the database.
- [new](new.md)
- [new](new_1.md)
- [resolve](resolve.md) — Resolve a pending HITL item (approved or denied) and notify the parked task.
- [resolve](resolve_1.md) — Resolve a pending HITL item (approved or denied) and notify the parked task.
- [submit_request](submit_request.md) — Register a pending HITL request and return a receiver channel that awaits human decision.
- [submit_request](submit_request_1.md) — Register a pending HITL request and return a receiver channel that awaits human decision.
