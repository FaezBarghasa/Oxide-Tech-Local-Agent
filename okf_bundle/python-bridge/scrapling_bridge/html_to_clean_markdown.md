---
okf_version: "0.2"
type: Function
title: _html_to_clean_markdown
resource: python-bridge/scrapling_bridge.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:scrapling_bridge.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:55:53Z"
concept_id: python-bridge/scrapling_bridge/html_to_clean_markdown
language: python
---

# _html_to_clean_markdown

## Signature

```python
def _html_to_clean_markdown(self, html: str, title: str = '') -> str
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `self` | `—` | `—` |

| `html` | `str` | `—` |

| `title` | `str` | `''` |

## Returns
`str`

## Source
Lines 196–209 in `python-bridge/scrapling_bridge.py`

## Relationships

| Type | Target |
|------|--------|
| related | [ScraplingBridge](/python-bridge/scrapling_bridge/ScraplingBridge.md) |
| called_by | [_fetch_scrapling](/python-bridge/scrapling_bridge/fetch_scrapling.md) |
