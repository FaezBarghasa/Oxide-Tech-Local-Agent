---
okf_version: "0.2"
type: Function
title: ExtractDocumentation
resource: python-bridge/server.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:server.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:56:12Z"
concept_id: python-bridge/server/ExtractDocumentation
language: python
---

# ExtractDocumentation

## Signature

```python
async def ExtractDocumentation(self, request, context)
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `self` | `—` | `—` |

| `request` | `—` | `—` |

| `context` | `—` | `—` |

## Source
Lines 34–42 in `python-bridge/server.py`

## Relationships

| Type | Target |
|------|--------|
| related | [PerceptionService](/python-bridge/server/PerceptionService.md) |
| calls | [fetch_url](/python-bridge/scrapling_bridge/fetch_url.md) |
