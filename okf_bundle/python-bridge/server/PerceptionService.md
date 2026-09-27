---
okf_version: "0.2"
type: Class
title: PerceptionService
resource: python-bridge/server.py
tags:
  - "lang:python"
  - "type:Class"
  - "module:python-bridge"
  - "domain:server.py"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-23T10:56:12Z"
concept_id: python-bridge/server/PerceptionService
language: python
---

# PerceptionService

## Inheritance

- `bridge_pb2_grpc.PerceptionServiceServicer if hasattr(bridge_pb2_grpc, 'PerceptionServiceServicer') else object`

## Methods

- `__init__`
- `ScrapeUrl`
- `ExtractDocumentation`

## Source
Lines 17–42 in `python-bridge/server.py`

## Relationships

| Type | Target |
|------|--------|
| related | [server](/python-bridge/server.md) |
| related | [__init__](/python-bridge/server/init.md) |
| related | [ScrapeUrl](/python-bridge/server/ScrapeUrl.md) |
| related | [ExtractDocumentation](/python-bridge/server/ExtractDocumentation.md) |
| called_by | [serve](/python-bridge/server/serve.md) |
