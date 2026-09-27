---
okf_version: "0.2"
type: Function
title: _fetch_scrapling
resource: python-bridge/scrapling_bridge.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:scrapling_bridge.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:55:53Z"
concept_id: python-bridge/scrapling_bridge/fetch_scrapling
language: python
---

# _fetch_scrapling

## Signature

```python
def _fetch_scrapling(self, url: str, selector: Optional[str] = None, css_clean: bool = True) -> ScrapeResult
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `self` | `—` | `—` |

| `url` | `str` | `—` |

| `selector` | `Optional[str]` | `None` |

| `css_clean` | `bool` | `True` |

## Returns
`ScrapeResult`

## Source
Lines 57–124 in `python-bridge/scrapling_bridge.py`

## Relationships

| Type | Target |
|------|--------|
| related | [ScraplingBridge](/python-bridge/scrapling_bridge/ScraplingBridge.md) |
| calls | [_html_to_clean_markdown](/python-bridge/scrapling_bridge/html_to_clean_markdown.md) |
| calls | [ScrapeResult](/python-bridge/scrapling_bridge/ScrapeResult.md) |
| calls | [_fetch_fallback](/python-bridge/scrapling_bridge/fetch_fallback.md) |
| called_by | [fetch_url](/python-bridge/scrapling_bridge/fetch_url.md) |
