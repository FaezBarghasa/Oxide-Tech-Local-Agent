# resource_gater

## Classs

- [GateRejection](GateRejection.md) — Rejection reason when work admission fails
- [GateStatus](GateStatus.md) — Circuit breaker state for admission control
- [ResourceGater](ResourceGater.md) — Dynamic Disk and VRAM circuit breaker

## Functions

- [admit_work](admit_work.md) — Check admission before accepting new agent turns or heavy allocations
- [admit_work](admit_work_1.md) — Check admission before accepting new agent turns or heavy allocations
- [current_status](current_status.md) — Current gate status
- [current_status](current_status_1.md) — Current gate status
- [default](default.md)
- [default](default_1.md)
- [get_available_disk_mb](get_available_disk_mb.md) — Read available disk space on the target filesystem in Megabytes using sysinfo
- [get_available_disk_mb](get_available_disk_mb_1.md) — Read available disk space on the target filesystem in Megabytes using sysinfo
- [new](new.md)
- [new](new_1.md)
- [poll_and_update](poll_and_update.md) — Poll system metrics, updating hysteresis circuit breaker
- [poll_and_update](poll_and_update_1.md) — Poll system metrics, updating hysteresis circuit breaker
- [test_resource_gater_hysteresis](test_resource_gater_hysteresis.md) — [tokio::test]
