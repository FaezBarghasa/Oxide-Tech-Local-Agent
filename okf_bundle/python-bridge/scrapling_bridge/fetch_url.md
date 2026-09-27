---
okf_version: "0.2"
type: Function
title: fetch_url
description: Fetch and parse a webpage using Scrapling (or fallback) with stealth headers.
resource: python-bridge/scrapling_bridge.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:scrapling_bridge.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:55:53Z"
concept_id: python-bridge/scrapling_bridge/fetch_url
language: python
---

# fetch_url

Fetch and parse a webpage using Scrapling (or fallback) with stealth headers.

## Signature

```python
def fetch_url(self, url: str, selector: Optional[str] = None, css_clean: bool = True) -> ScrapeResult
```

## Docstring

Fetch and parse a webpage using Scrapling (or fallback) with stealth headers.

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
Lines 50–55 in `python-bridge/scrapling_bridge.py`

## Relationships

| Type | Target |
|------|--------|
| related | [ScraplingBridge](/python-bridge/scrapling_bridge/ScraplingBridge.md) |
| calls | [_fetch_scrapling](/python-bridge/scrapling_bridge/fetch_scrapling.md) |
| calls | [_fetch_fallback](/python-bridge/scrapling_bridge/fetch_fallback.md) |
| called_by | [ExtractDocumentation](/python-bridge/server/ExtractDocumentation.md) |
| called_by | [ScrapeUrl](/python-bridge/server/ScrapeUrl.md) |
