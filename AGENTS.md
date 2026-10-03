# Oxide-Tech Local Agent OS: Agent Guidelines & Instructions

For complete architectural specifications, crate topological interfaces, and formal invariants, refer to:
- [`docs/LLM_AGENT_CONTEXT_PRIMER.md`](docs/LLM_AGENT_CONTEXT_PRIMER.md)
- [`docs/CRATE_TOPOLOGY_AND_INTERFACES.md`](docs/CRATE_TOPOLOGY_AND_INTERFACES.md)
- [`docs/DOMAIN_CONTROLLER_IPC_REFERENCE.md`](docs/DOMAIN_CONTROLLER_IPC_REFERENCE.md)
- [`docs/NATIVE_TOOLING_AND_PROMPT_GRAMMAR.md`](docs/NATIVE_TOOLING_AND_PROMPT_GRAMMAR.md)
- [`docs/OXIDE_PROTOCOL_AND_WIRE_SPEC.md`](docs/OXIDE_PROTOCOL_AND_WIRE_SPEC.md)
- [`docs/FORMAL_VERIFICATION_AND_INVARIANTS.md`](docs/FORMAL_VERIFICATION_AND_INVARIANTS.md)
- [`docs/MULTI_PHYSICS_AND_SIMULATION_SPEC.md`](docs/MULTI_PHYSICS_AND_SIMULATION_SPEC.md)
- [`docs/MEMORY_FABRIC_AND_GRAPH_SCHEMA.md`](docs/MEMORY_FABRIC_AND_GRAPH_SCHEMA.md)

---

## Hard Rules for AI Agents

1. **Search Tool**: ALWAYS use `oxide-embed search "<query>" --stair` as the only code search tool.
2. **Monolith Target**: Single production binary target `oxide-tech-local-agent` (crate `src-tauri`).
3. **Pure Rust**: 100% offline-first, zero cloud egress, zero Python runtime dependencies.
4. **Rust 2024 Invariants**: Zero `.unwrap()` in production paths; use typed `Result<T, E>`.
5. **Quality Gate**: Must pass `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --test pure_rust_stack_test`.
6. **Communication**: Zero filler words; diff-style targeted code edits; clickable markdown links with `file://` scheme.
