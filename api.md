# Oxide-Tech Local Agent OS: API Specifications

The Oxide-Tech Local Agent OS exposes endpoints over TCP (HTTP/1.1, HTTP/2, WebSockets) and UDP (QUIC/HTTP/3) on port `8080`, with native `llama-server` inference orchestrated on port `8081` and local Ollama bridge on port `11434`.

---

## 1. Gateway & Control Plane Endpoints (`:8080`)

### A. Health & Diagnostics
- **`GET /health/live`**: Liveness probe returning `{ "status": "alive" }`.
- **`GET /health/ready`**: Readiness probe checking SurrealDB, Qdrant, and native inference engine connections.
- **`GET /metrics`**: Prometheus metrics for token latency, GPU offload memory, and active agent sessions.

### B. Universal AI Gateway & Routing Endpoints
- **`POST /api/v1/gateway/route`**:
  - Dispatches prompt to optimal provider using one of 19 routing strategies (`priority`, `fill-first`, `weighted`, `round-robin`, `p2c`, `least-used`, `random`, `strict-random`, `cost-optimized`, `headroom`, `reset-window`, `reset-aware`, `context-relay`, `context-optimized`, `cache-optimized`, `lkgp`, `auto`, `fusion`, `chaos`).
  - Request:
    ```json
    {
      "prompt": "Analyze SPI DMA buffer safety in no_std",
      "model": "auto",
      "strategy": "cost-optimized",
      "temperature": 0.2
    }
    ```
  - Response:
    ```json
    {
      "selected_provider": "local_llama_server",
      "model_id": "Ternary-Bonsai-2-27B-Abliterated-PTQ1_0.gguf",
      "response": "...",
      "tokens_used": 342,
      "latency_ms": 128.4
    }
    ```
- **`GET /api/v1/gateway/accounts`**: Returns active provider accounts, daily token usage, RPM/RPD limits, and circuit health.
- **`POST /v1/chat/completions`**: OpenAI-compatible chat completion proxying directly to active local `llama-server` or Ollama instance.
- **`POST /api/agent/think`**: High-level agent reasoning endpoint delegating to active native engine.

### C. The Oxide Protocol & Distributed Transactions
- **`POST /api/dtx/begin`**:
  - Request: `{ "title": "Optimize IoT enclosure", "initiator": "local-agent", "domains": ["firmware_ide", "oxide_eda", "oxide_3d"] }`
  - Response: `{ "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6", "status": "pending" }`
- **`POST /api/dtx/commit`**:
  - Request: `{ "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6" }`
  - Response: `{ "status": "committed" }`
- **`POST /api/dtx/rollback`**:
  - Request: `{ "dtx_id": "0192a3b4-c5d6-7e8f-9a0b-c1d2e3f4a5b6", "reason": "Silicon exceeded 85C" }`
  - Response: `{ "status": "rolled_back" }`

### D. Cross-Domain Multi-Physics Verification
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

### E. Graph Engineering & Impact Analysis
- **`POST /api/graph/parse`**:
  - Request: `{ "file_path": "src/driver.rs", "content": "..." }`
  - Response: `{ "nodes": [...], "edges": [...] }`
- **`POST /api/graph/impact`**:
  - Request: `{ "modified_node": "fn:src/driver.rs:init_dma" }`
  - Response: `{ "blast_radius": 4, "affected_files": ["src/main.rs", "src/bus.rs"], "recommended_tests": ["cargo test --test dma_test"] }`
- **`POST /api/graph/prune`**:
  - Request: `{ "focal_node": "fn:src/driver.rs:init_dma", "max_hops": 2 }`
  - Response: `{ "pruned_subgraph_json": "...", "token_savings_pct": 82.4 }`

### F. Streamable HTTP & Real-Time Reasoning (SSE)
- **`POST /api/agent/stream`**:
  - Request: `{ "prompt": "Verify SPI clock prescaler on STM32F4", "mode": "code" }`
  - Response (Server-Sent Events `text/event-stream`):
    - `data: {"type": "thought_chunk", "content": "1. Checking APB1 clock..."}`
    - `data: {"type": "content_chunk", "content": "The APB1 clock is running at 42MHz..."}`
    - `data: {"type": "done"}`

---

## 2. Desktop-First Tauri v2 IPC Command Matrix (`src-tauri`)

The native desktop application connects directly to core engine modules via strongly typed `#[tauri::command]` IPC handlers:

### A. Model Management & Native Inference (`model_ipc.rs`)
- `scan_default_local_gguf_models()` $\to$ `Vec<ModelInfo>`: Recursively scans disk (`~/models`, `~/.cache/huggingface`, `~/.ollama/models`, `/opt/models`) for real `.gguf` weights.
- `get_active_model()` $\to$ `Option<ActiveModelInfo>`: Returns active loaded model and GPU offload status.
- `set_active_model(model_name: String, provider: String)` $\to$ `ActiveModelInfo`: Spawns or reconfigures the local `llama-server` process with `-ngl 99`.
- `unload_active_model()` $\to$ `bool`: Gracefully terminates the running `llama-server` child process.
- `get_model_stats(model_name: String)` $\to$ `ModelStats`: Returns total prompt/completion tokens, GPU utilization, and execution latency.
- `chat_with_model(prompt: String, max_tokens: Option<u32>, temperature: Option<f32>)` $\to$ `ChatResponse`: Proxies prompt directly to active native runtime.

### B. Hardware & System Doctor (`doctor.rs`)
- `doctor_run_diagnostics()` $\to$ `DoctorReportDto`: Scans probe-rs targets, Qdrant/SurrealDB health, and Linux permissions.
- `doctor_install_udev_rules()` $\to$ `UdevInstallResultDto`: Safely installs hardware debug probe rules to `/etc/udev/rules.d/69-probe-rs.rules`.

### C. RE-Forge Studio Binary Analysis (`reforge_ipc.rs`)
- `reforge_analyze_file(request: ReforgeFileRequestDto)` $\to$ `ReforgeAnalysisResultDto`: Zero-copy ELF/PE/Mach-O header analysis, ARM Cortex-M Vector Table (`IvtEntryDto`) decoding, RTOS signature detection, and Shannon entropy calculation.

### D. Deterministic Verifier Matrix (`verifier_ipc.rs`)
- `verifier_run_suite(request: VerifierRunRequestDto)` $\to$ `VerifierRunResultDto`: Executes unit tests, formal verification, or hardware simulation suites with atomic git stash checkpoints.
- `verifier_export_evidence(bundle: EvidenceBundleDto)` $\to$ `String`: Exports signed cryptographic evidence bundles.

### E. Memory, GraphRAG & Rules Fabric (`memory.rs`)
- `memory_conflicts()` $\to$ `Vec<ConflictReportDto>`: Identifies contradictory project rules and architectural decisions.
- `memory_explain(symbol: String, hops: u32)` $\to$ `GraphExplanationDto`: Computes multi-hop topological call graphs and blast radius.
- `memory_remember(content: String, kind: String, tags: Vec<String>)` $\to$ `MemoryRecordDto`: Persists scoped architectural rules.
- `memory_recall(query: String, limit: usize)` $\to$ `Vec<MemoryRecordDto>`: Retrieves relevant project knowledge.

### F. Profile & Configuration (`config_ipc.rs`)
- `config_read()` $\to$ `ConfigDto`: Reads local `config.toml` parameters.
- `config_save(content: String)` $\to$ `ConfigSaveResultDto`: Validates and writes profile changes in real time.
