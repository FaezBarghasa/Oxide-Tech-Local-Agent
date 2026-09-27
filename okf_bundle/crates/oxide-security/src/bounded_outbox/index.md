# bounded_outbox

## Classs

- [BoundedDiagnosticOutbox](BoundedDiagnosticOutbox.md) — Bounded, non-blocking telemetry outbox that drops records on full queue without blocking caller
- [DiagnosticRecord](DiagnosticRecord.md) — [derive(Debug, Clone, Serialize, Deserialize)]

## Functions

- [dropped_records_count](dropped_records_count.md)
- [dropped_records_count](dropped_records_count_1.md)
- [emit](emit.md) — Non-blocking send: if queue is full, increments dropped_count and returns immediately
- [emit](emit_1.md) — Non-blocking send: if queue is full, increments dropped_count and returns immediately
- [log_file_path](log_file_path.md)
- [log_file_path](log_file_path_1.md)
- [new](new.md)
- [new](new_1.md)
- [test_bounded_diagnostic_outbox](test_bounded_diagnostic_outbox.md) — [tokio::test]
