# Domain Controller IPC Reference (Tauri v2 Facade)

This document provides a comprehensive technical reference for the **Domain Controller Facade Pattern** implemented in `src-tauri/src/controllers/` and exposed to the desktop UI via Tauri v2 IPC commands.

---

## 1. Controller Facade Architecture

To eliminate IPC sprawl and ensure modular separation of concerns, 18 legacy command handlers in `src-tauri/src/` have been consolidated into 4 domain controllers:

```
src-tauri/src/controllers/
├── mod.rs                   # Re-exports and unified controller registration
├── agent_controller.rs      # Autonomous ReAct reasoning, native tools, and routing
├── system_controller.rs     # Target diagnostics, probe-rs udev rules, and configs
├── workspace_controller.rs  # STAIR Code-ToC leaf search, Memanto memory, and conflicts
└── forge_controller.rs      # Binary reverse engineering, PTX decompilation, and verifiers
```

All IPC handlers return `Result<T, String>` and strictly adhere to asynchronous non-blocking semantics. Blocking operations (e.g. recursive disk weight scans) are offloaded to `tokio::task::spawn_blocking`.

---

## 2. Controller Command Catalogs

### A. AgentController ([`src-tauri/src/controllers/agent_controller.rs`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/src-tauri/src/controllers/agent_controller.rs))

Handles the cognitive reasoning loop, model querying, prompt generation, and zero-latency in-process tool dispatch.

| Command Name | Parameters | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `agent_think` | `prompt: String`, `session_id: Option<String>` | `ThinkResponse` | Runs an autonomous ReAct reasoning cycle. |
| `agent_execute_native_tool` | `tool_name: String`, `parameters: Value` | `Value` | Dispatches tool call directly to in-process `NativeToolRegistry`. |
| `agent_route_prompt` | `prompt: String`, `strategy: Option<String>` | `RouteResponse` | Routes query via Universal AI Gateway (19 strategies). |
| `agent_generate_prompt` | `system: String`, `user: String`, `tools: Vec<String>` | `String` | Formats system and tools prompt using Ornith XML grammar. |
| `scan_local_gguf_models` | None | `Vec<GgufModelRecord>` | Non-blocking recursive scan for local `.gguf` weights. |
| `query_ollama_models` | None | `Vec<OllamaModelRecord>` | Queries running Ollama daemon for active models. |
| `query_sglang_models` | None | `Vec<SglangModelRecord>` | Queries SGLang server for active served endpoints. |

#### TypeScript Invocation Example:
```typescript
import { invoke } from '@tauri-apps/api/core';

// Execute native tool without network/IPC serialization penalty
const result = await invoke<Record<string, unknown>>('agent_execute_native_tool', {
  toolName: 'stair_search',
  parameters: { query: 'apply_rope', max_results: 5 }
});
```

---

### B. SystemController ([`src-tauri/src/controllers/system_controller.rs`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/src-tauri/src/controllers/system_controller.rs))

Supervises hardware diagnostics, target microcontroller connectivity, and configuration management.

| Command Name | Parameters | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `doctor_run_diagnostics` | None | `DiagnosticReport` | Verifies compiler toolchains, `oxide-embed`, and GPU drivers. |
| `doctor_install_udev` | None | `UdevInstallReport` | Installs system udev rules for CMSIS-DAP and ST-Link probes. |
| `doctor_probe_hardware` | None | `HardwareProbeReport` | Scans USB and SWD buses for connected microcontrollers. |
| `config_load_profile` | `profile: String` | `AppConfig` | Dynamically loads a profile (`Lite`, `Standard`, `Pro`, etc.). |
| `config_save_profile` | `config: AppConfig` | `()` | Persists configuration updates to `~/.config/oxide-tech/config.toml`. |

---

### C. WorkspaceController ([`src-tauri/src/controllers/workspace_controller.rs`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/src-tauri/src/controllers/workspace_controller.rs))

Bridges desktop frontend to `oxide-embed` project memory, STAIR Code-ToC, and Memanto governance.

| Command Name | Parameters | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `memory_stair_search` | `query: String`, `max_results: Option<usize>` | `Vec<StairResult>` | Hierarchical Code-ToC leaf search with enclosing breadcrumbs. |
| `memory_recall` | `topic: String` | `Vec<MemoryRecord>` | Recalls architectural decisions and verified invariants. |
| `memory_remember` | `content: String`, `kind: String` | `String` | Records a new decision or architectural constraint into Memanto. |
| `memory_conflicts` | None | `Vec<ConflictReport>` | Audits semantic memory for conflicting decisions or rules. |

---

### D. ForgeController ([`src-tauri/src/controllers/forge_controller.rs`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/src-tauri/src/controllers/forge_controller.rs))

Provides deep binary reverse engineering, GPU kernel decompilation, and formal verification.

| Command Name | Parameters | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `reforge_analyze_binary` | `path: String`, `arch: Option<String>` | `BinaryAnalysis` | Extracts ELF/Mach-O sections, entropy, and ARM vector tables. |
| `reforge_decompile_ptx` | `path: String` | `PtxDecompileResult` | Zero-copy decompiler parsing CUDA PTX / cuDNN assembly. |
| `verifier_run_suite` | `workspace_path: String` | `VerificationReport` | Executes real-time compilation, ERC, and CAD clearance checks. |
| `verifier_export_evidence`| `workspace_path: String` | `EvidenceBundle` | Generates a cryptographically signed Evidence Bundle. |

---

## 3. Error Handling Invariants

1. **Zero `.unwrap()` in Handlers**: All controller methods catch panics and format errors into human-readable strings:
   ```rust
   pub async fn agent_execute_native_tool(
       tool_name: String,
       parameters: serde_json::Value,
   ) -> Result<serde_json::Value, String> {
       let registry = NativeToolRegistry::default_suite();
       registry.execute(&tool_name, parameters).await.map_err(|e| e.to_string())
   }
   ```
2. **Backward Compatibility Guarantee**: Legacy IPC commands registered in `src-tauri/src/main.rs` maintain their original command strings, routing internally through these controllers to prevent frontend breaks.
