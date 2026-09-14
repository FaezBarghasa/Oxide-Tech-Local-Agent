# 🚀 Quickstart: Oxide-Tech Local Agent OS

Welcome to **Oxide-Tech Local Agent OS** — a local-first, graph-aware, sandboxed agentic engineering system designed for deterministic software, embedded systems, open-source EDA, and native binary reverse engineering.

---

## 1. System Requirements & Diagnostics

Before running Oxide-Tech, verify your host environment:

```bash
# Run the automated diagnostics
./scripts/doctor.sh
```

### Optional: Install Hardware Probe udev Rules
If flashing physical microcontrollers (STM32, ESP32, nRF52) via `probe-rs`:
```bash
./scripts/install_udev_rules.sh
```

---

## 2. Choosing an Operating Mode

Oxide-Tech adapts to your local hardware setup:

### 🟢 Mode A: Lite Mode (CPU / Low-VRAM Laptops)
Runs using local **Ollama** or **llama.cpp** and lightweight embedded storage:
```bash
# 1. Pull the recommended coding model in Ollama
ollama pull qwen2.5-coder:7b

# 2. Start the Gateway in Lite Mode
cargo run -p gateway -- --profile lite
```

### 🔵 Mode B: Standard Workstation Mode (Single GPU)
Runs using single-GPU local inference with SurrealDB & Qdrant:
```bash
# 1. Launch databases via Docker Compose
docker compose up -d surrealdb qdrant

# 2. Start the Gateway in Standard Mode
cargo run -p gateway -- --profile standard
```

### 🟣 Mode C: Pro Workstation Mode (Dual GPU)
Runs high-throughput SGLang tensor parallelism with deep reasoning models:
```bash
cargo run -p gateway -- --profile pro
```

---

## 3. Launching the Studio UI

The web-based visual control plane provides real-time agent loops, HITL approval inboxes, AST knowledge graph exploration, and verifier logs.

```bash
cd ui/oxide-agent-studio
pnpm install
pnpm dev
```
Open **http://localhost:5173** to access Oxide Studio.

---

## 4. Key Workflows & CLI

### Inspect Repository Architecture
```bash
cargo run -p gateway -- graph inspect --target .
```

### Run Closed-Loop Verification
```bash
# Test embedded kernel boot in QEMU sandbox
cargo test -p mcp-qemu-redox

# Verify workspace safety and gatekeeper policies
cargo test -p mcp-cargo-gatekeeper
```

---

## 5. Security & HITL Model

- **Read-Only / Sandboxed by Default**: Commands run inside isolated Bubblewrap (`bwrap`) containers without network egress.
- **Human-in-the-Loop (HITL)**: High-impact actions (e.g. flashing microcontrollers, workspace file writes, git pushes) require one-click approval in the Studio UI before execution.
