# Oxide Agent Studio

**Oxide Agent Studio** is the unified web control plane and visual IDE for **Oxide-Tech-Local-Agent**, built with React 19, TypeScript, TailwindCSS, Lucide icons, and Vite.

---

## 1. Core Feature Tabs

- **Model Catalog & Native Engine (`CatalogTab.tsx`)**:
  - Live discovery of local `.gguf` weights on host storage (`~/models`, `~/.cache/huggingface`, `~/.ollama/models`).
  - Real-time GPU layer offloading configuration (`-ngl 99`), context size allocation, and on-demand `llama-server` lifecycle management.
- **Universal Gateway Dashboard (`GatewayTab.tsx`)**:
  - Configuration of 19 algorithmic routing strategies (`priority`, `cost-optimized`, `context-relay`, `auto`, etc.).
  - Upstream browser-authenticated account management and token bucket gauges.
- **Graph Topology & Structural Skeleton (`GraphTopologyTab.tsx`)**:
  - Live interactive visualization of multi-modal code graphs (AST nodes, function callers/callees).
  - Downstream blast radius impact analysis and predictive test target selection.
  - Subgraph context pruning visualizer demonstrating 75%–89% prompt token savings.
- **Supervisor & Agent Orchestration (`AgentDashboard.tsx`)**:
  - Multi-persona DAG handoff monitor (`Lead Architect` $\rightarrow$ `Firmware Coder` $\rightarrow$ `DRC Reviewer`).
  - Real-time ReAct loop execution status and oscillation guard indicator.
- **Verifier & Sandboxes (`VerifierTab.tsx`)**:
  - Deterministic compiler and DRC verifiers (`cargo check`, `kicad-cli`, `qemu`).
  - Atomic working tree checkpoint history and instant one-click rollbacks.
- **Project Memory & GraphRAG (`MemoryTab.tsx`)**:
  - Integrated `oxide-embed` semantic memory fabric, STAIR Code-ToC search, and contradiction auditor.
- **Terminal & Ring Buffers (`TerminalTab.tsx`)**:
  - Low-latency streaming of ephemeral terminal session logs, dirty editor buffers, and stack traces.

---

## 2. Running Locally

> [!IMPORTANT]
> Always use `pnpm` for package management in this directory.

```bash
# 1. Install dependencies
pnpm install

# 2. Build production assets & server
pnpm build

# 3. Start local server on port 3000
pnpm start
```

The application will be accessible at: **[http://localhost:3000](http://localhost:3000)** or directly via the Tauri v2 Desktop binary `oxide-agent`.
