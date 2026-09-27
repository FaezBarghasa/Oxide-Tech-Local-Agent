---
okf_version: "0.2"
type: Function
title: serve
resource: python-bridge/server.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:server.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:56:12Z"
concept_id: python-bridge/server/serve
language: python
---

# serve

## Signature

```python
async def serve(port: int = 50051, uds_path: str = '/tmp/oxide_bridge.sock')
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `port` | `int` | `50051` |

| `uds_path` | `str` | `'/tmp/oxide_bridge.sock'` |

## Source
Lines 59–116 in `python-bridge/server.py`

## Relationships

| Type | Target |
|------|--------|
| related | [server](/python-bridge/server.md) |
| calls | [add_BridgeServiceServicer_to_server](/python-bridge/bridge_pb2_grpc/add_BridgeServiceServicer_to_server.md) |
| calls | [BridgeService](/python-bridge/server/BridgeService.md) |
| calls | [KiCadBridgeServicer](/python-bridge/kicad_bridge/KiCadBridgeServicer.md) |
| calls | [CADServiceServicer](/python-bridge/blender_bridge/CADServiceServicer.md) |
| calls | [PerceptionService](/python-bridge/server/PerceptionService.md) |
| called_by | [main](/crates/mcp-cargo-gatekeeper/src/main/main.md) |
| called_by | [connect_stdio](/crates/mcp-clients/src/lib/connect_stdio.md) |
| called_by | [main](/crates/mcp-live-docs/src/main/main.md) |
| called_by | [main](/crates/mcp-probe-rs/src/main/main.md) |
| called_by | [main](/crates/mcp-qemu-redox/src/main/main.md) |
| called_by | [main](/python-bridge/server/main.md) |
