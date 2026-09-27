---
okf_version: "0.2"
type: Class
title: ScrapeResult
resource: python-bridge/scrapling_bridge.py
tags:
  - "lang:python"
  - "type:Class"
  - "module:python-bridge"
  - "domain:scrapling_bridge.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:55:53Z"
concept_id: python-bridge/scrapling_bridge/ScrapeResult
language: python
---

# ScrapeResult

## Decorators

- `dataclass`

## Fields

| Name | Type | Visibility |
|------|------|------------|
| `url` | `str` | `` |
| `success` | `bool` | `` |
| `status_code` | `int` | `` |
| `title` | `str` | `` |
| `markdown_content` | `str` | `` |
| `links` | `List[str]` | `` |
| `metadata` | `Dict[str, str]` | `` |
| `error` | `Optional[str]` | `` |

## Source
Lines 29–37 in `python-bridge/scrapling_bridge.py`

## Relationships

| Type | Target |
|------|--------|
| related | [scrapling_bridge](/python-bridge/scrapling_bridge.md) |
| called_by | [_fetch_fallback](/python-bridge/scrapling_bridge/fetch_fallback.md) |
| called_by | [_fetch_scrapling](/python-bridge/scrapling_bridge/fetch_scrapling.md) |
