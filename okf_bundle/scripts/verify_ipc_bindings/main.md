---
okf_version: "0.2"
type: Function
title: main
resource: scripts/verify_ipc_bindings.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:scripts"
  - "domain:verify_ipc_bindings.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T14:02:47Z"
concept_id: scripts/verify_ipc_bindings/main
language: python
---

# main

## Signature

```python
def main()
```

## Source
Lines 60–78 in `scripts/verify_ipc_bindings.py`

## Relationships

| Type | Target |
|------|--------|
| related | [verify_ipc_bindings](/scripts/verify_ipc_bindings.md) |
| calls | [find_frontend_invocations](/scripts/verify_ipc_bindings/find_frontend_invocations.md) |
| calls | [find_tauri_registered_handlers](/scripts/verify_ipc_bindings/find_tauri_registered_handlers.md) |
