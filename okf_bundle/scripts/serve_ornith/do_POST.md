---
okf_version: "0.2"
type: Function
title: do_POST
resource: scripts/serve_ornith.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:scripts"
  - "domain:serve_ornith.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T09:00:27Z"
concept_id: scripts/serve_ornith/do_POST
language: python
---

# do_POST

## Signature

```python
def do_POST(self)
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `self` | `—` | `—` |

## Source
Lines 59–103 in `scripts/serve_ornith.py`

## Relationships

| Type | Target |
|------|--------|
| related | [OrnithHandler](/scripts/serve_ornith/OrnithHandler.md) |
| calls | [_send_json](/scripts/serve_ornith/send_json.md) |
