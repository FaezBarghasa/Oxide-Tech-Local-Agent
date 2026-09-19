# 🚀 Quickstart: Oxide-Tech Local Agent OS

Welcome to **Oxide-Tech Local Agent OS** — a local-first, graph-aware, sandboxed agentic engineering system designed for deterministic software, embedded systems, open-source EDA, native binary reverse engineering, and multi-physics co-simulation.

---

## 1. System Requirements & Diagnostics

Before running Oxide-Tech, verify your host environment using the unified CLI:

```bash
# Run the automated diagnostics
cargo run -- doctor
# Or using the installed binary:
oxide-agent doctor
```

### Optional: Install Hardware Probe udev Rules
If flashing physical microcontrollers (STM32, ESP32, nRF52) via `probe-rs`:
```bash
sudo ./scripts/install_udev_rules.sh
```

---

## 2. Choosing an Operating Mode

Oxide-Tech adapts to your local hardware setup:

### 🟢 Mode A: Lite Mode (CPU / Low-VRAM Laptops)
Runs using local **Ollama** or **llama.cpp** and lightweight embedded storage:
```bash
# 1. Pull the recommended coding model in Ollama
ollama pull qwen2.5-coder:7b

# 2. Start the Daemon in Lite Mode
oxide-agent daemon --profile lite --port 8080
```

### 🔵 Mode B: Standard Workstation Mode (Single GPU)
Runs using single-GPU local inference with SurrealDB & Qdrant:
```bash
# 1. Launch databases via Docker Compose
docker compose up -d surrealdb qdrant

# 2. Start the Daemon in Standard Mode
oxide-agent daemon --profile standard --port 8080
```

### 🟣 Mode C: Pro Workstation Mode (Dual GPU)
Runs high-throughput SGLang tensor parallelism with deep reasoning models:
```bash
oxide-agent daemon --profile pro --port 8080
```

---

## 3. Launching the Studio UI

The web-based visual control plane provides real-time agent loops, HITL approval inboxes, AST knowledge graph exploration, and multi-physics verifier logs.

```bash
# Launch Studio UI directly
oxide-agent studio --port 3000
```
Open **http://localhost:3000** to access Oxide Studio.

---

## 4. Key Workflows & CLI Subcommands

### High-Throughput Binary Reverse Engineering & Decompilation
```bash
oxide-agent re-forge path/to/binary --arch x86_64 --decompile
```

### Polyglot to Rust Refactoring & Synthesis
```bash
oxide-agent forge-rust path/to/source.py --out ./refactored_rust --verify
```

### Deterministic Verifier & Evidence Bundling
```bash
oxide-agent verify --workspace . --export-evidence ./target/evidence
```

### Probe Daemon Health
```bash
oxide-agent status --gateway-url http://127.0.0.1:8080
```

---

## 5. The Oxide Protocol & Multi-Physics Verification

- **Distributed Transactions (UUIDv7 DTX)**: All operations across Firmware (IDE), Electronics (EDA), and Mechanics (CAD) run under atomic distributed transaction management with automated `rollback_dtx` on verification failure.
- **Electro-Thermal-Mechanical Co-Simulation**: Automatically tests whether firmware thermal loads violate physical enclosure clearance or exceed silicon junction thresholds ($85^\circ\text{C}$).
