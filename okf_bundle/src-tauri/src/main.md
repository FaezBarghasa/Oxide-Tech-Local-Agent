---
okf_version: "0.2"
type: Module
title: src
description: oxide-tech-local-agent — Universal Single Production Binary.
resource: src-tauri/src/main.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:src-tauri"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:20:55Z"
concept_id: src-tauri/src/main
language: rust
---

# src

oxide-tech-local-agent — Universal Single Production Binary.

## Docstring

oxide-tech-local-agent — Universal Single Production Binary.

One binary runs everything:
- `oxide-tech-local-agent` (no args) or `oxide-tech-local-agent desktop` → Tauri desktop window
with the embedded gateway (background thread), full workstation UI, and oxide-embed memory.
- `oxide-tech-local-agent daemon [--config PATH]` → headless gateway service (systemd unit).
- `oxide-tech-local-agent doctor`                 → comprehensive environment diagnostics.
- `oxide-tech-local-agent re-forge <file>`         → pure-Rust binary / PTX GPU reverse engineering.
- `oxide-tech-local-agent verify [--workspace .]`  → deterministic verifier suite & evidence bundle.
- `oxide-tech-local-agent memory <args...>` / `embed <args...>` → STAIR Code-ToC & Memanto memory.
- `oxide-tech-local-agent status`                 → probe the local gateway.

## Relationships

| Type | Target |
|------|--------|
| related | [usage](/src-tauri/src/main/usage.md) |
| related | [flag_value](/src-tauri/src/main/flag_value.md) |
| related | [run_desktop](/src-tauri/src/main/run_desktop.md) |
| related | [gateway_status](/src-tauri/src/main/gateway_status.md) |
| related | [run_doctor_cli](/src-tauri/src/main/run_doctor_cli.md) |
| related | [run_reforge_cli](/src-tauri/src/main/run_reforge_cli.md) |
| related | [run_verify_cli](/src-tauri/src/main/run_verify_cli.md) |
| related | [run_memory_passthrough](/src-tauri/src/main/run_memory_passthrough.md) |
| related | [run_status](/src-tauri/src/main/run_status.md) |
| related | [main](/src-tauri/src/main/main.md) |
| related | [anyhow](/_dependencies/cargo/anyhow.md) |
