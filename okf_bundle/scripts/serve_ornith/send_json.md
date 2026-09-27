---
okf_version: "0.2"
type: Function
title: _send_json
resource: scripts/serve_ornith.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:scripts"
  - "domain:serve_ornith.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T09:00:27Z"
concept_id: scripts/serve_ornith/send_json
language: python
---

# _send_json

## Signature

```python
def _send_json(self, data, status = 200)
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `self` | `—` | `—` |

| `data` | `—` | `—` |

| `status` | `—` | `200` |

## Source
Lines 34–41 in `scripts/serve_ornith.py`

## Relationships

| Type | Target |
|------|--------|
| related | [OrnithHandler](/scripts/serve_ornith/OrnithHandler.md) |
| calls | [encode](/crates/oxide-network/src/crypto/encode.md) |
| called_by | [do_GET](/scripts/serve_ornith/do_GET.md) |
| called_by | [do_OPTIONS](/scripts/serve_ornith/do_OPTIONS.md) |
| called_by | [do_POST](/scripts/serve_ornith/do_POST.md) |
