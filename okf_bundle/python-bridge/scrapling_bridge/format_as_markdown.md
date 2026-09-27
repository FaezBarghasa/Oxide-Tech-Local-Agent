---
okf_version: "0.2"
type: Function
title: _format_as_markdown
resource: python-bridge/scrapling_bridge.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:scrapling_bridge.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:55:53Z"
concept_id: python-bridge/scrapling_bridge/format_as_markdown
language: python
---

# _format_as_markdown

## Signature

```python
def _format_as_markdown(self, text: str, title: str, url: str) -> str
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `self` | `—` | `—` |

| `text` | `str` | `—` |

| `title` | `str` | `—` |

| `url` | `str` | `—` |

## Returns
`str`

## Source
Lines 211–214 in `python-bridge/scrapling_bridge.py`

## Relationships

| Type | Target |
|------|--------|
| related | [ScraplingBridge](/python-bridge/scrapling_bridge/ScraplingBridge.md) |
| called_by | [_fetch_fallback](/python-bridge/scrapling_bridge/fetch_fallback.md) |
