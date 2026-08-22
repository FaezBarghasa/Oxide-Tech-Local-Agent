# Oxide-Tech Local Agent OS: API Specifications

The Oxide-Tech Local Agent OS exposes endpoints over TCP (HTTP/1.1, HTTP/2, WebSockets) and UDP (QUIC/HTTP/3). All endpoints bind to port `8080`.

---

## 1. System Endpoints

### A. Health & Diagnostics

#### Liveness Probe
- **Path**: `GET /health/live`
- **Response**: `200 OK`
- **Format**: `application/json`
```json
{
  "status": "alive",
  "timestamp": "2026-06-21T13:00:00Z"
}
```

#### Readiness Probe
Checks connection health to SurrealDB, Qdrant, and the local vLLM instance.
- **Path**: `GET /health/ready`
- **Response**: `200 OK` (when all check pass) or `503 Service Unavailable`
- **Format**: `application/json`
```json
{
  "status": "ready",
  "checks": {
    "database": { "status": "ok" },
    "qdrant": { "status": "ok" },
    "vllm": { "status": "ok" }
  }
}
```

#### Prometheus Metrics
- **Path**: `GET /metrics`
- **Response**: `200 OK`
- **Format**: `text/plain; version=0.0.4`
```text
# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 12.34
```

---

### B. Workspace Operations

#### Repository Structure
Retrieves a recursive JSON tree representation of the active workspace directory, omitting large binary assets and git objects.
- **Path**: `GET /api/repository/structure`
- **Query Parameters**:
  - `workspace_path` (string, optional): Absolute path to explore.
- **Response**: `200 OK`
```json
{
  "name": "Oxide-Tech-Local-Agent",
  "path": "",
  "is_dir": true,
  "children": [
    {
      "name": "Cargo.toml",
      "path": "Cargo.toml",
      "is_dir": false
    },
    {
      "name": "crates",
      "path": "crates",
      "is_dir": true,
      "children": [
        {
          "name": "api",
          "path": "crates/api",
          "is_dir": true
        }
      ]
    }
  ]
}
```

---

### C. Sandboxed Cargo Execution

#### Cargo Check
Executes `cargo check` inside the sandboxed environment.
- **Path**: `POST /api/cargo/check`
- **Payload**:
```json
{
  "workspace_path": "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent"
}
```
- **Response**: `200 OK` or `500 Internal Server Error`
```json
{
  "status": "success",
  "exit_code": 0,
  "stdout": "",
  "stderr": "   Finished dev [unoptimized + debuginfo] target(s) in 0.05s"
}
```

#### Cargo Clippy
Executes `cargo clippy` inside the sandboxed environment.
- **Path**: `POST /api/cargo/clippy`
- **Payload**: Same as Cargo Check.
- **Response**: Same format as Cargo Check.

---

### D. Tree-Sitter Code Indexing

#### Parse File and Workspace Symbols
Scans the workspace directory for Rust files, extracts class/trait/method signatures, and saves them to SurrealDB.
- **Path**: `POST /api/tree-sitter/parse`
- **Payload**:
```json
{
  "workspace_path": "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent"
}
```
- **Response**: `200 OK`
```json
{
  "status": "success",
  "files_parsed": 12,
  "symbols_indexed": 142
}
```

---

### E. CAD & Simulation Tools

#### KiCad: Load Board
Loads and checks structural properties of a KiCad PCB layout.
- **Path**: `POST /api/kicad/load-board`
- **Payload**:
```json
{
  "board_path": "design/rpi_hat.kicad_pcb",
  "workspace_path": "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent"
}
```
- **Response**: `200 OK`
```json
{
  "status": "success",
  "message": "KiCad board loaded successfully: design/rpi_hat.kicad_pcb",
  "file_size": 40960
}
```

#### KiCad: Run DRC
Executes KiCad Design Rule Check (DRC) inside the sandbox.
- **Path**: `POST /api/kicad/run-drc`
- **Payload**: Same as Load Board.
- **Response**: `200 OK`
```json
{
  "status": "success",
  "exit_code": 0,
  "stdout": "DRC completed with 0 errors, 0 warnings (mocked)",
  "stderr": ""
}
```

#### SKiDL: Generate Netlist
Generates circuit netlist connections from python scripts.
- **Path**: `POST /api/skidl/generate`
- **Payload**:
```json
{
  "script_path": "scripts/gen_schematic.py",
  "workspace_path": "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent"
}
```
- **Response**: `200 OK`
```json
{
  "status": "success",
  "exit_code": 0,
  "stdout": "Netlist generated successfully.",
  "stderr": ""
}
```

#### Thermal: Simulate Heat Dissipation
Executes a thermal simulator script inside the sandbox.
- **Path**: `POST /api/thermal/simulate`
- **Payload**:
```json
{
  "board_path": "design/rpi_hat.kicad_pcb",
  "workspace_path": "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent"
}
```
- **Response**: `200 OK`
```json
{
  "status": "warning",
  "message": "Thermal simulation package not found or failed, returning mock simulation report.",
  "exit_code": 0,
  "stdout": "Thermal simulation complete. Max temperature: 62.5C at U1. Board operating temperatures within safe bounds.",
  "stderr": ""
}
```

---

### F. Multi-Agent Code Generation

#### Trigger Agent Loop
Main entrance to trigger the Chief Planner and Execution auto-healing loop.
- **Path**: `POST /api/agent/generate`
- **Payload**:
```json
{
  "prompt": "Implement a new SPI device driver in crates/sandbox/src/spi_driver.rs",
  "workspace_path": "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent"
}
```
- **Response**: `200 OK`
```json
{
  "status": "PASSED",
  "message": "Ready for deployment.",
  "goal": "Implement a new SPI device driver in crates/sandbox/src/spi_driver.rs"
}
```

---

## 2. WebSocket Real-time Streams

### A. Compilation Progress
Connect to this socket to stream compiler updates in real-time.
- **URL**: `ws://127.0.0.1:8080/ws/compilation`
- **Output Format**: Text frames containing cargo stdout/stderr lines.

### B. Agent Progress
Connect to this socket to stream the current thinking processes and plan executions.
- **URL**: `ws://127.0.0.1:8080/ws/agent-progress`
- **Output Format**: Text frames describing planning phases (e.g. Chief Planner executing step 2, code diffs being applied).

---

## 3. Blog & Knowledge Curation Endpoints

### A. Blog Web Pages

#### Blog Home Page (RTL Farsi)
- **Path**: `GET /blog` or `GET /blog/`
- **Query Parameters**:
  - `page` (integer, optional): Page index (default: 1).
- **Response**: `200 OK`
- **Format**: `text/html; charset=utf-8`

#### Blog Post View
- **Path**: `GET /blog/post/{id}`
- **Response**: `200 OK`
- **Format**: `text/html; charset=utf-8`

#### Tag Page
- **Path**: `GET /blog/tag/{tag}`
- **Response**: `200 OK`
- **Format**: `text/html; charset=utf-8`

### B. Feeds & JSON APIs

#### Blog Atom/RSS Feed
- **Path**: `GET /blog/feed.xml`
- **Response**: `200 OK`
- **Format**: `application/atom+xml; charset=utf-8`

#### Blog JSON API
Retrieves list of curated blog posts in JSON format.
- **Path**: `GET /blog/api/posts`
- **Response**: `200 OK`
- **Format**: `application/json`

#### Manual Knowledge Ingestion Trigger
Triggers the daily background scraping, embedding, and blog curation pipeline manually.
- **Path**: `POST /api/knowledge/update`
- **Headers**:
  - `Authorization: Bearer <JWT>` (Requires Developer/Admin role)
- **Response**: `202 Accepted`
- **Format**: `text/plain`
- **Response Body**: `"Manual update cycle started in the background"`
