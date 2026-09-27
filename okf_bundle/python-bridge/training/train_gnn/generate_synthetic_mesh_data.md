---
okf_version: "0.2"
type: Function
title: generate_synthetic_mesh_data
description: Generates synthetic PCB/IC thermal mesh topology for surrogate training.
resource: python-bridge/training/train_gnn.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T07:56:38Z"
concept_id: python-bridge/training/train_gnn/generate_synthetic_mesh_data
language: python
---

# generate_synthetic_mesh_data

Generates synthetic PCB/IC thermal mesh topology for surrogate training.

## Signature

```python
def generate_synthetic_mesh_data(num_nodes: int = 64, ambient_temp: float = 25.0) -> Dict[str, Any]
```

## Docstring

Generates synthetic PCB/IC thermal mesh topology for surrogate training.

## Parameters

| Name | Type | Default |
|------|------|---------|
| `num_nodes` | `int` | `64` |

| `ambient_temp` | `float` | `25.0` |

## Returns
`Dict[str, Any]`

## Source
Lines 92–136 in `python-bridge/training/train_gnn.py`

## Relationships

| Type | Target |
|------|--------|
| related | [train_gnn](/python-bridge/training/train_gnn.md) |
| called_by | [train_thermal_gnn](/python-bridge/training/train_gnn/train_thermal_gnn.md) |
