# Oxide-Tech Local Agent OS: API Specifications

The Oxide-Tech Local Agent OS exposes endpoints over TCP (HTTP/1.1, HTTP/2, WebSockets) and UDP (QUIC/HTTP/3) on port `8080`, with local SGLang inference on port `30000`.

---

## 1. Gateway & Control Plane Endpoints (`:8080`)

### A. Health & Diagnostics
- **`GET /health/live`**: Liveness probe returning `{ "status": "alive" }`.
- **`GET /health/ready`**: Readiness probe checking SurrealDB, Qdrant, and SGLang connections.
- **`GET /metrics`**: Prometheus metrics for token latency, GPU offload memory, and active agent sessions.

### B. Graph Engineering & Impact Analysis
- **`POST /api/graph/parse`**:
  - Request: `{ "file_path": "src/driver.rs", "content": "..." }`
  - Response: `{ "nodes": [...], "edges": [...] }`
- **`POST /api/graph/impact`**:
  - Request: `{ "modified_node": "fn:src/driver.rs:init_dma" }`
  - Response: `{ "blast_radius": 4, "affected_files": ["src/main.rs", "src/bus.rs"], "recommended_tests": ["cargo test --test dma_test"] }`
- **`POST /api/graph/prune`**:
  - Request: `{ "focal_node": "fn:src/driver.rs:init_dma", "max_hops": 2 }`
  - Response: `{ "pruned_subgraph_json": "...", "token_savings_pct": 82.4 }`

### C. Multi-Agent Orchestration & Planning
- **`POST /api/agent/plan`**:
  - Request: `{ "goal": "Build STM32 DMA SPI driver in no_std", "context": "..." }`
  - Response: `{ "thought": "...", "steps": [{ "step_id": 1, "description": "...", "assigned_persona": "Architect", "tool_calls": [...] }] }`

---

## 2. Inference & Dynamic LoRA Serving Endpoints (`:30000`)

### A. Dynamic LoRA Activation
- **`POST /v1/lora/activate`**:
  - Request: `{ "adapter_name": "lora_embedded_rust_v2", "action": "activate" }`
  - Response: `{ "status": "success", "active_adapter": "lora_embedded_rust_v2" }`

### B. OpenAI-Compatible Chat Completions
- **`POST /v1/chat/completions`**:
  - Request: `{ "model": "ornith-1.0-9b", "messages": [...], "temperature": 0.1, "max_tokens": 512 }`
  - Response: standard OpenAI completion format with latency metadata.

---

## 3. Self-Evolution Endpoints

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
