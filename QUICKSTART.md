# 🚀 Quickstart: Oxide-Tech Local Agent OS

Welcome to **Oxide-Tech Local Agent OS** — a local-first, graph-aware, sandboxed agentic engineering system designed for deterministic software, embedded systems, open-source EDA, native binary reverse engineering, multi-physics co-simulation, and universal multi-provider AI model orchestration.

---

## 1. System Requirements & Diagnostics

Before running Oxide-Tech, verify your host environment using the unified CLI:

```bash
# Run the automated diagnostics
cargo run -- doctor
# Or using the single installed binary:
oxide-tech-local-agent doctor
```

### Optional: Install Hardware Probe udev Rules
If flashing physical microcontrollers (STM32, ESP32, nRF52) via `probe-rs`:
```bash
sudo oxide-tech-local-agent doctor --install-udev
```

---

## 2. Choosing an Operating Mode & Inference Backend

Oxide-Tech adapts to your local hardware setup and supports both **native offline weights** and **universal cloud routing**:

### 🟢 Mode A: Native Offline GGUF Engine (Zero-Stub `llama-server`)
Runs directly on your host GPU/CPU with zero cloud dependency:
```bash
# 1. Place .gguf models in ~/models or ~/.cache/huggingface
# Supported models: Ternary-Bonsai-2-27B, DeepSeek-R1-8B, gemma4-v2, etc.

# 2. Start the Daemon with native model orchestrator
oxide-tech-local-agent daemon --profile standard --port 8080
```

### 🔵 Mode B: Local Ollama Bridge
Connects to an existing Ollama daemon:
```bash
# 1. Pull the recommended coding model in Ollama
ollama pull qwen2.5-coder:7b

# 2. Start the Daemon in Lite Mode
oxide-tech-local-agent daemon --profile lite --port 8080
```

### 🟣 Mode C: Universal AI Gateway (19 Routing Strategies)
Route queries dynamically across multiple browser-authenticated accounts and local engines:
```bash
# 1. Start the Universal AI Gateway
oxide-tech-local-agent daemon --profile enterprise --port 8080

# 2. Test dynamic routing (e.g. priority, context-relay, or cost-optimized)
curl -X POST http://127.0.0.1:8080/api/v1/gateway/route \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Analyze SPI DMA buffer safety in no_std", "model": "auto", "strategy": "cost-optimized"}'
```

---

## 3. Launching the Desktop Application (Studio UI)

Oxide-Tech Local Agent is designed **Desktop-First**. The single hermetic binary `oxide-tech-local-agent` runs the desktop window with embedded background gateway and inlined frontend assets:
- **Doctor Tab**: Hardware connectivity, toolchain checks, and udev rules installer.
- **Catalog Tab**: Real-time GGUF model manager, GPU layer offloading (`-ngl`), and native `llama-server` lifecycle.
- **Gateway Tab**: Universal router policy configuration (19 strategies), browser authentication status, and token buckets.
- **RE-Forge Tab**: Interactive vector table inspection, entropy graphing, and decompilation.
- **Verifier Tab**: Real-time multi-suite testing & cryptographic evidence bundle export.
- **Memory Tab**: `oxide-embed` semantic graph, STAIR Code-ToC leaf search, and memory recall.
- **Settings Tab**: Fast profile switching (`Lite`, `Standard`, `Pro`, `AirGapped`, `Enterprise`).

```bash
# Launch Native Desktop Studio (Single hermetic binary)
cargo run
# Or via installed binary:
oxide-tech-local-agent desktop
```

---

## 4. Key Workflows & CLI Subcommands (Headless Automation)

### High-Throughput Binary Reverse Engineering & Decompilation
```bash
oxide-tech-local-agent re-forge path/to/binary --arch x86_64
```

### Deterministic Verifier & Evidence Bundling
```bash
oxide-tech-local-agent verify --workspace . --json
```

### STAIR Code-ToC & Memanto Memory Passthrough
```bash
oxide-tech-local-agent memory search "my_function" --stair
oxide-tech-local-agent memory recall "architectural decision"
```

### Probe Daemon Health
```bash
oxide-tech-local-agent status --gateway-url http://127.0.0.1:8080
```

---

## 5. The Oxide Protocol & Multi-Physics Verification

- **Distributed Transactions (UUIDv7 DTX)**: All operations across Firmware (IDE), Electronics (EDA), and Mechanics (CAD) run under atomic distributed transaction management with automated `rollback_dtx` on verification failure.
- **Electro-Thermal-Mechanical Co-Simulation**: Automatically tests whether firmware thermal loads violate physical enclosure clearance or exceed silicon junction thresholds ($85^\circ\text{C}$).
