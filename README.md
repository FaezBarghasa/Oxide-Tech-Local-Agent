# Oxide-Tech Local Agent OS

Oxide-Tech Local Agent OS is a production-grade, polyglot (Rust/Python) backend environment built to orchestrate next-generation hardware engineering tasks. The platform coordinates code compilation, AST analysis, schematic generation, 3D PCB layout verification, thermal simulations, and multi-agent AI execution loops, backed by a real-time event system.

---

## Technical Stack & Architecture

- **Gateway Layer**: Parallel execution of **Actix-web 4.x** (TCP on port `8080`) and native **QUIC / HTTP/3** (UDP on port `8080`), configured with secure TLS 1.3.
- **Database Layer**: **SurrealDB v3** as a multi-model graph database for hardware relationships, and **Qdrant v1.18.x** for dense/sparse hybrid semantic vector search on datasheet collections.
- **AI/LLM Inference**: **vLLM (0.5.x)** serving local open-weights LLMs (such as Qwen/Gemma) for structured schematic and code generation.
- **Execution Sandboxing**: A strict, containerized sandbox layer utilizing Docker/Morph, preventing arbitrary execution of generated scripts directly on the host system.
- **Telemetry & Monitoring**: Prometheus metrics exported on `/metrics`, alongside standard liveness (`/health/live`) and readiness (`/health/ready`) probes.

---

## Project Structure

```text
Oxide-Tech-Local-Agent/
├── Cargo.toml               # Workspace configuration and shared dependencies
├── crates/
│   ├── api/                 # Actix-web / HTTP/3 server and WebSocket endpoints
│   ├── tree-sitter-service/ # Tree-sitter Rust AST parser & symbol extractor
│   ├── surrealdb-service/   # SurrealDB v3 driver, schema models, and graph queries
│   ├── qdrant-service/      # Qdrant client, dense/sparse collection setups
│   ├── vllm-client/         # vLLM API client for local LLM completion & streaming
│   └── sandbox/             # Docker sandbox environment for safe code execution
├── python-bridge/           # Python-based bridges for KiCad, Skidl, and ML models
│   ├── training/            # PyTorch / Unsloth fine-tuning scripts
│   ├── server.py            # Local Python services router
│   └── requirements.txt     # Python environment requirements
├── scripts/                 # Initialize scripts, migrations, and assets
└── target/                  # Compiled Rust artifacts
```

---

## Quickstart Guide

### 1. Prerequisites
- **Rust**: Rust 1.75+ or newer.
- **Docker**: For running sandboxed executions.
- **SurrealDB**: Version 3.x installed and running.
- **Qdrant**: Version 1.18.x or newer running.

### 2. Configure Environment Variables
Create a `.env` file in the project root or export the following variables in your shell:
```bash
export SURREALDB_URL="ws://127.0.0.1:8000" # Or mem:// for in-memory DB testing
export QDRANT_URL="http://127.0.0.1:6334"
export VLLM_API_URL="http://127.0.0.1:8000"
```

### 3. Initialize Databases
Initialize database schemas and mock collections using the provided setup scripts:
```bash
# Initialize SurrealDB Schema
surreal import --conn http://127.0.0.1:8000 --user root --pass root --ns oxide_tech --db main scripts/init-surrealdb.surql

# Initialize Qdrant Collections
curl -X PUT http://127.0.0.1:6333/collections/datasheets -H "Content-Type: application/json" -d @scripts/init-qdrant.json
```

### 4. Build and Run
Build the monorepo workspace to ensure everything compiles cleanly:
```bash
cargo build --release
```

Run the backend server:
```bash
cargo run --bin api
```

The gateway will start listening on:
- **HTTP/1.1 & HTTP/2**: `http://127.0.0.1:8080` (TCP)
- **HTTP/3 (QUIC)**: `https://127.0.0.1:8080` (UDP)

---

## Verification & Testing

### Automated Checks
Execute the Rust test suite to verify code correctness and schema integrations:
```bash
cargo test --workspace
```

Ensure lint checks and compiler warnings are verified:
```bash
cargo clippy --all-targets -- -D warnings
```

---

## Detailed Documentation

For deep technical insights into the backend platform, explore the following documentation:
- [System Architecture Details](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/architecture.md): Visualizes the multi-layered layout, services isolation, and polyglot architecture.
- [Mermaid Diagrams](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/diagram.md): Flowcharts of runtime components and the AI agent auto-healing loop.
- [Changelog](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/changelog.md): Highlights SurrealDB v3 upgrades, QUIC addition, and thread-safety details.
- [API Documentation](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/api.md): Standardized schema endpoints, WebSockets, and UDP HTTP/3 routing.
- [Execution Flows](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/flow.md): Walkthrough of the Chief Planner-Execution-Verifier lifecycle.
