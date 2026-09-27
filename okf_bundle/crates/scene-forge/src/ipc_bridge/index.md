# ipc_bridge

## Classs

- [BlenderBridge](BlenderBridge.md) — Binary IPC bridge using length-prefixed `postcard` framing.
- [BlenderCommand](BlenderCommand.md) — Commands dispatched across the Postcard binary IPC bridge to the 3D host/addon.
- [BlenderResponse](BlenderResponse.md) — Responses returned from the 3D host across the binary IPC stream.
- [BridgeSupervisor](BridgeSupervisor.md) — Bidirectional heartbeat supervisor for the Python subprocess bridge
- [MockBlenderEngine](MockBlenderEngine.md) — In-memory mock 3D engine simulator for deterministic testing and ReAct loops.
- [PrimitiveType](PrimitiveType.md) — 3D primitive geometry type.
- [ShmGeometryBuffer](ShmGeometryBuffer.md) — POSIX Shared Memory Descriptor for Zero-Copy Large Geometries (>2MB)

## Functions

- [allocate](allocate.md) — Create or open named shared memory segment in `/dev/shm`
- [allocate](allocate_1.md) — Create or open named shared memory segment in `/dev/shm`
- [decode_command](decode_command.md) — Helper to decode a length-prefixed payload buffer.
- [decode_command](decode_command_1.md) — Helper to decode a length-prefixed payload buffer.
- [encode_command](encode_command.md) — Helper to encode a command into length-prefixed bytes.
- [encode_command](encode_command_1.md) — Helper to encode a command into length-prefixed bytes.
- [encode_response](encode_response.md) — Helper to encode a response into length-prefixed bytes (for server/mock implementation).
- [encode_response](encode_response_1.md) — Helper to encode a response into length-prefixed bytes (for server/mock implementation).
- [exceeds_zero_copy_threshold](exceeds_zero_copy_threshold.md) — Check if payload exceeds zero-copy threshold (2 MB)
- [exceeds_zero_copy_threshold](exceeds_zero_copy_threshold_1.md) — Check if payload exceeds zero-copy threshold (2 MB)
- [execute](execute.md) — Dispatch a command over the binary stream and read the response.
- [execute](execute_1.md) — Dispatch a command over the binary stream and read the response.
- [handle_command](handle_command.md) — Process a command and return the corresponding response.
- [handle_command](handle_command_1.md) — Process a command and return the corresponding response.
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [record_heartbeat_failure](record_heartbeat_failure.md) — Record a failed heartbeat and check if process must be terminated and respawned
- [record_heartbeat_failure](record_heartbeat_failure_1.md) — Record a failed heartbeat and check if process must be terminated and respawned
- [record_heartbeat_success](record_heartbeat_success.md) — Reset failure count on successful Pong
- [record_heartbeat_success](record_heartbeat_success_1.md) — Reset failure count on successful Pong
- [terminate_hung_process](terminate_hung_process.md) — Terminate unresponsive child process via SIGKILL
- [terminate_hung_process](terminate_hung_process_1.md) — [cfg(not(target_os = "linux"))]
- [terminate_hung_process](terminate_hung_process_2.md) — Terminate unresponsive child process via SIGKILL
- [terminate_hung_process](terminate_hung_process_3.md) — [cfg(not(target_os = "linux"))]
- [test_heartbeat_encode_decode](test_heartbeat_encode_decode.md) — [test]
- [test_shm_threshold](test_shm_threshold.md) — [test]
- [test_supervisor_watchdog_threshold](test_supervisor_watchdog_threshold.md) — [test]
