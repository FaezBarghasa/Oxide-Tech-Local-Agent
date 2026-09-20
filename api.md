# Oxide-Tech Local Agent OS: API Specifications

The Oxide-Tech Local Agent OS exposes endpoints over TCP (HTTP/1.1, HTTP/2, WebSockets) and UDP (QUIC/HTTP/3) on port `8080`, with local SGLang inference on port `30000`.

---

## 1. Gateway & Control Plane Endpoints (`:8080`)

### A. Health & Diagnostics
- **`GET /health/live`**: Liveness probe returning `{ "status": "alive" }`.
- **`GET /health/ready`**: Readiness probe checking SurrealDB, Qdrant, and SGLang connections.
- **`GET /metrics`**: Prometheus metrics for token latency, GPU offload memory, and active agent sessions.

### B. The Oxide Protocol & Distributed Transactions
- **`POST /api/dtx/begin`**:
  - Request: `{ "title": "Optimize IoT enclosure", "initiator": "local-agent", "domains": ["firmware_ide", "oxide_eda", "oxide_3d"] }`
  - Response: `{ "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6", "status": "pending" }`
- **`POST /api/dtx/commit`**:
  - Request: `{ "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6" }`
  - Response: `{ "status": "committed" }`
- **`POST /api/dtx/rollback`**:
  - Request: `{ "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6", "reason": "Silicon exceeded 85C" }`
  - Response: `{ "status": "rolled_back" }`

### C. Cross-Domain Multi-Physics Verification
- **`POST /api/verify/co-simulation`**:
  - Request:
    ```json
    {
      "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6",
      "firmware_duty_cycle": 0.85,
      "mcu_base_watts": 1.2,
      "enclosure_material": "Aluminum_6061"
    }
    ```
  - Response:
    ```json
    {
      "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6",
      "passed": true,
      "firmware_power_watts": 1.056,
      "peak_temperature_c": 57.3,
      "clearance_margin_mm": 2.0,
      "recommended_action": null
    }
    ```

### D. Graph Engineering & Impact Analysis
- **`POST /api/graph/parse`**:
  - Request: `{ "file_path": "src/driver.rs", "content": "..." }`
  - Response: `{ "nodes": [...], "edges": [...] }`
- **`POST /api/graph/impact`**:
  - Request: `{ "modified_node": "fn:src/driver.rs:init_dma" }`
  - Response: `{ "blast_radius": 4, "affected_files": ["src/main.rs", "src/bus.rs"], "recommended_tests": ["cargo test --test dma_test"] }`
- **`POST /api/graph/prune`**:
  - Request: `{ "focal_node": "fn:src/driver.rs:init_dma", "max_hops": 2 }`
  - Response: `{ "pruned_subgraph_json": "...", "token_savings_pct": 82.4 }`

### E. Streamable HTTP & Real-Time Reasoning (SSE)
- **`POST /api/agent/stream`**:
  - Request: `{ "prompt": "Verify SPI clock prescaler on STM32F4", "mode": "code" }`
  - Response (Server-Sent Events `text/event-stream`):
    - `data: {"type": "thought_chunk", "content": "1. Checking APB1 clock..."}`
    - `data: {"type": "content_chunk", "content": "The APB1 clock is running at 42MHz..."}`
    - `data: {"type": "done"}`

### F. Human-in-the-Loop (HITL) Inbox & Adversarial Safety
- **`GET /api/inbox/pending`**:
  - Response: `[{ "id": "req-987", "task_id": "task-123", "agent_id": "coder-1", "action": "probe-rs flash --chip STM32F407VG", "risk_class": "Flash", "status": "Pending", "reviewer_score": 0.92, "reviewer_verdict": "PASS" }]`
- **`POST /api/inbox/resolve`**:
  - Request: `{ "id": "req-987", "approved": true, "feedback": "Target verified on testbench" }`
  - Response: `{ "status": "resolved", "id": "req-987" }`

---

## 2. Self-Evolution Endpoints

### A. Delta Harvester Record
- **`POST /api/self-evolve/record-delta`**:
  - Request:
    ```json
    {
      "prompt": "Implement safe UART buffer",
      "failed_code": "fn send(buf: &[u8]) { ... }",
      "fixed_code": "fn send(buf: &[u8]) -> Result<(), Error> { ... }",
      "compiler_log": "error[E0308]: mismatched types"
    }
    ```
  - Response: `{ "status": "recorded", "pool": "grpo_training_pool" }`

### B. JIT Tool Synthesis
- **`POST /api/self-evolve/synthesize-tool`**:
  - Request: `{ "tool_name": "fast_crc32_simd", "language": "mojo", "script": "..." }`
  - Response: `{ "status": "verified", "sandbox": "bwrap", "mounted_path": "/tmp/jit_tools/fast_crc32_simd.mojo" }`

---

## 3. Polyglot to Rust Refactoring Endpoints (`crates/forge-rust`)

### A. Refactor Codebase to Idiomatic Rust
- **`POST /api/forge-rust/refactor`**:
  - Request:
    ```json
    {
      "source": "package main\n\ntype MotorState struct {\n    RPM int\n    Enabled bool\n}\n\nfunc (m *MotorState) SetSpeed(targetRPM int) error {\n    return nil\n}",
      "module_name": "motor_ctrl",
      "language_hint": "Go",
      "verify_syntax": true,
      "is_binary": false
    }
    ```
  - Response:
    ```json
    {
      "source_language": "Go",
      "rust_code": "pub struct MotorState {\n    pub rpm: isize,\n    pub enabled: bool,\n}\n\nimpl MotorState {\n    pub fn set_speed(&mut self, target_rpm: isize) -> Result<(), anyhow::Error> {\n        Ok(())\n    }\n}\n",
      "files": {
        "Cargo.toml": "[package]\nname = \"motor_ctrl\"\n...",
        "src/lib.rs": "..."
      }
    }
    ```

### B. Language Detection & Inspection
- **`POST /api/forge-rust/detect`**:
  - Request: `{ "source": "#include <stdio.h>\nint main() { return 0; }", "file_path": "main.c" }`
  - Response: `{ "detected_language": "C" }`

---

## 4. Desktop-First Tauri v2 IPC Command Matrix (`src-tauri`)

The native desktop application connects directly to core engine modules via strongly typed `#[tauri::command]` IPC handlers:

### A. Hardware & System Doctor
- `doctor_run_diagnostics()` $\to$ `DoctorReportDto`: Scans probe-rs targets, Qdrant/SurrealDB health, and Linux permissions.
- `doctor_install_udev_rules()` $\to$ `UdevInstallResultDto`: Safely installs hardware debug probe rules to `/etc/udev/rules.d/69-probe-rs.rules`.

### B. RE-Forge Studio Binary Analysis
- `reforge_analyze_file(request: ReforgeFileRequestDto)` $\to$ `ReforgeAnalysisResultDto`:
  - Zero-copy ELF/PE/Mach-O header analysis.
  - ARM Cortex-M Vector Table (`IvtEntryDto`) decoding.
  - RTOS signature detection (`FreeRTOS`, `Embassy`, `RTIC`, `Zephyr`).
  - Shannon byte entropy graphing across binary chunks.
  - Neural/Safe-Rust decompilation output.

### C. Deterministic Verifier Matrix
- `verifier_run_suite(request: VerifierRunRequestDto)` $\to$ `VerifierRunResultDto`:
  - Executes unit tests, formal verification, or hardware simulation suites.
  - Automatically captures git stash rollback checkpoints.
- `verifier_export_evidence(bundle: EvidenceBundleDto)` $\to$ `String`: Exports signed, timestamped cryptographic proof bundles.

### D. Memory, GraphRAG & Rules Fabric
- `memory_conflicts()` $\to$ `Vec<ConflictReportDto>`: Identifies contradictory project rules and architectural decisions.
- `memory_explain(symbol: String, hops: u32)` $\to$ `GraphExplanationDto`: Computes multi-hop topological call graphs and blast radius.
- `memory_remember(content: String, kind: String, tags: Vec<String>)` $\to$ `MemoryRecordDto`: Persists scoped architectural rules.
- `memory_recall(query: String, limit: usize)` $\to$ `Vec<MemoryRecordDto>`: Retrieves relevant project knowledge.

### E. Profile & Configuration
- `config_read()` $\to$ `ConfigDto`: Reads local `config.toml` parameters.
- `config_save(content: String)` $\to$ `ConfigSaveResultDto`: Validates and writes profile changes in real time.

