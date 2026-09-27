---
okf_version: "0.2"
type: Function
title: _fetch_fallback
resource: python-bridge/scrapling_bridge.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:scrapling_bridge.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:55:53Z"
concept_id: python-bridge/scrapling_bridge/fetch_fallback
language: python
---

# _fetch_fallback

## Signature

```python
def _fetch_fallback(self, url: str, selector: Optional[str] = None, css_clean: bool = True, error_prefix: str = '') -> ScrapeResult
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `self` | `—` | `—` |

| `url` | `str` | `—` |

| `selector` | `Optional[str]` | `None` |

| `css_clean` | `bool` | `True` |

| `error_prefix` | `str` | `''` |

## Returns
`ScrapeResult`

## Source
Lines 126–194 in `python-bridge/scrapling_bridge.py`

## Relationships

| Type | Target |
|------|--------|
| related | [ScraplingBridge](/python-bridge/scrapling_bridge/ScraplingBridge.md) |
| calls | [ScrapeResult](/python-bridge/scrapling_bridge/ScrapeResult.md) |
| calls | [_format_as_markdown](/python-bridge/scrapling_bridge/format_as_markdown.md) |
| called_by | [_fetch_scrapling](/python-bridge/scrapling_bridge/fetch_scrapling.md) |
| called_by | [fetch_url](/python-bridge/scrapling_bridge/fetch_url.md) |
