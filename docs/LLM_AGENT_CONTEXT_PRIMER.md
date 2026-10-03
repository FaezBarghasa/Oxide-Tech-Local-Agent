# LLM Agent Context Primer & System Axioms

This document serves as the primary system prompt primer and context grounding manual for any autonomous AI agent or LLM operating within the **Oxide-Tech Local Agent OS** repository.

---

## 1. Core System Axioms & Constraints

1. **User Identity**: Assisting Faez Barghasa, senior embedded & systems engineer.
2. **Environment & Host**: Pop!_OS 24.04 LTS (Cosmic). Offline-first, token-lean, resilient engineering workflows.
3. **Monolith Executable Target**: Single production binary at `target/release/oxide-tech-local-agent` (crate `src-tauri`). Default workspace member (`default-members = ["src-tauri"]`).
4. **Pure-Rust & Zero Cloud Egress**: 100% Rust 2024 Edition (`1.85+`, `resolver = "3"`). Zero external cloud dependencies. Zero external Python runtime dependencies.
5. **Quality Gate**: The entire workspace must compile cleanly with zero compiler warnings and pass `cargo clippy --workspace --all-targets -- -D warnings`.

---

## 2. Mandatory Search & Context Rules (`oxide-embed`)

Traditional grep/ripgrep dumping is prohibited in favor of AST-aware hierarchical retrieval:

- **Mandatory Search Tool**: ALWAYS use `oxide-embed search "<query>" --stair` (STAIR Code-ToC leaf search) for locating functions, structs, and implementations.
- **Surgical Symbol Retrieval**: Use `oxide-embed read <path> --symbol <symbol_name>` to slice exact target symbols instead of viewing full multi-thousand-line files.
- **Knapsack Task Context**: Use `oxide-embed context "<task_description>" --budget 1500` to pack relevant cross-crate context within strict token budgets.
- **Topological Graph Queries**:
  - `oxide-embed callers <symbol>`
  - `oxide-embed callees <symbol>`
  - `oxide-embed impact <symbol>`
- **Autonomous Project Indexing**: If `.oxide/` is missing or out of sync, immediately execute:
  ```bash
  oxide-embed init && oxide-embed index --device auto
  ```

---

## 3. Rust Engineering & Code Invariants

- **Zero `.unwrap()` in Production**: Never use `.unwrap()` or `.expect()` in non-test code. Use `Result<T, E>` with typed error enums (`thiserror`).
- **Async Runtime Hygiene**: Never perform blocking I/O (file system traversal, GGUF weight parsing, synchronous sleep) on Tokio async worker threads. Offload to `tokio::task::spawn_blocking`.
- **Tauri IPC Controller Facade**: All frontend desktop IPC commands must route through the 4 domain controllers under [`src-tauri/src/controllers/`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/src-tauri/src/controllers/):
  - `AgentController`: Reasoning, model querying, and native tool execution.
  - `SystemController`: Hardware probes, diagnostics, and config.
  - `WorkspaceController`: STAIR Code-ToC and Memanto memory.
  - `ForgeController`: Binary RE, PTX decompilation, and verifiers.
- **Native In-Process Tooling**: Core tools must implement `NativeTool` in [`crates/oxide-tooling`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/crates/oxide-tooling) and register in `NativeToolRegistry` for zero-IPC dispatch.

---

## 4. Verification Checkpoint Commands

Before reporting any task complete or committing changes, execute:

```bash
# 1. Format verification
cargo fmt --check

# 2. Strict linter quality gate (zero warnings allowed)
cargo clippy --workspace --all-targets -- -D warnings

# 3. Horizon verification integration suite (Horizons 0 through VII)
cargo test --test pure_rust_stack_test

# 4. Real-time UX & latency benchmark
cargo test --test realtime_user_experience_test

# 5. Full workspace unit and doc-tests
cargo test --workspace

# 6. AST & Knowledge Engine diagnostics
oxide-embed doctor
```

---

## 5. Communication & Output Rules

- **Zero Filler**: No introductory or concluding conversational filler ("Sure, I can help with that", "I hope this helps!").
- **Diff-Style Edits**: Provide targeted diffs omitting unchanged lines and obvious imports.
- **Clickable Markdown Links**: All file and symbol references MUST use the `file://` URI scheme (e.g. [`src-tauri/src/main.rs`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/src-tauri/src/main.rs)).
