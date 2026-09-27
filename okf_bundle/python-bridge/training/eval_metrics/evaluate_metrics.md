---
okf_version: "0.2"
type: Function
title: evaluate_metrics
resource: python-bridge/training/eval_metrics.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-22T11:37:05Z"
concept_id: python-bridge/training/eval_metrics/evaluate_metrics
language: python
---

# evaluate_metrics

## Signature

```python
def evaluate_metrics(samples: list[dict]) -> dict
```

## Parameters

| Name | Type | Default |
|------|------|---------|
| `samples` | `list[dict]` | `—` |

## Returns
`dict`

## Source
Lines 7–18 in `python-bridge/training/eval_metrics.py`

## Relationships

| Type | Target |
|------|--------|
| related | [eval_metrics](/python-bridge/training/eval_metrics.md) |
| called_by | [main](/python-bridge/training/eval_metrics/main.md) |
