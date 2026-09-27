---
okf_version: "0.2"
type: Module
title: verify_ipc_bindings
description: Verify Tauri IPC Bindings Integrity
resource: scripts/verify_ipc_bindings.py
tags:
  - "lang:python"
  - "type:Module"
  - "module:scripts"
  - "domain:verify_ipc_bindings.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T14:02:47Z"
concept_id: scripts/verify_ipc_bindings
language: python
---

# verify_ipc_bindings

Verify Tauri IPC Bindings Integrity

## Docstring

Verify Tauri IPC Bindings Integrity
Validates that every command invoked in the frontend (TS/TSX) is implemented and registered in src-tauri/src/

## Relationships

| Type | Target |
|------|--------|
| related | [find_frontend_invocations](/scripts/verify_ipc_bindings/find_frontend_invocations.md) |
| related | [find_tauri_registered_handlers](/scripts/verify_ipc_bindings/find_tauri_registered_handlers.md) |
| related | [main](/scripts/verify_ipc_bindings/main.md) |
