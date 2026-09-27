---
okf_version: "0.2"
type: Function
title: RunThermalSimulation
resource: python-bridge/bridge_pb2_grpc.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:bridge_pb2_grpc.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-07-25T15:09:31Z"
concept_id: python-bridge/bridge_pb2_grpc/RunThermalSimulation_1
language: python
---

# RunThermalSimulation

## Signature

```python
def RunThermalSimulation(request, target, options = (), channel_credentials = None, call_credentials = None, insecure = False, compression = None, wait_for_ready = None, timeout = None, metadata = None)
```

## Decorators

- `staticmethod`

## Parameters

| Name | Type | Default |
|------|------|---------|
| `request` | `—` | `—` |

| `target` | `—` | `—` |

| `options` | `—` | `()` |

| `channel_credentials` | `—` | `None` |

| `call_credentials` | `—` | `None` |

| `insecure` | `—` | `False` |

| `compression` | `—` | `None` |

| `wait_for_ready` | `—` | `None` |

| `timeout` | `—` | `None` |

| `metadata` | `—` | `None` |

## Source
Lines 85–99 in `python-bridge/bridge_pb2_grpc.py`

## Relationships

| Type | Target |
|------|--------|
| related | [BridgeService](/python-bridge/bridge_pb2_grpc/BridgeService.md) |
