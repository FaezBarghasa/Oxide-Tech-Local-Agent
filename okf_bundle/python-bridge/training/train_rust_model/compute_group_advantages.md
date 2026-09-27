---
okf_version: "0.2"
type: Function
title: compute_group_advantages
description: "Computes GRPO relative advantage: A_i = (r_i - mean(r)) / (std(r) + 1e-8)"
resource: python-bridge/training/train_rust_model.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-22T11:51:29Z"
concept_id: python-bridge/training/train_rust_model/compute_group_advantages
language: python
---

# compute_group_advantages

Computes GRPO relative advantage: A_i = (r_i - mean(r)) / (std(r) + 1e-8)

## Signature

```python
def compute_group_advantages(rewards: list[float]) -> list[float]
```

## Docstring

Computes GRPO relative advantage: A_i = (r_i - mean(r)) / (std(r) + 1e-8)

## Parameters

| Name | Type | Default |
|------|------|---------|
| `rewards` | `list[float]` | `—` |

## Returns
`list[float]`

## Source
Lines 32–42 in `python-bridge/training/train_rust_model.py`

## Relationships

| Type | Target |
|------|--------|
| related | [train_rust_model](/python-bridge/training/train_rust_model.md) |
| called_by | [main](/python-bridge/training/train_rust_model/main.md) |
