---
okf_version: "0.2"
type: Function
title: train_thermal_gnn
description: Trains the GNN surrogate model until thermal error converges below tolerance.
resource: python-bridge/training/train_gnn.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T07:56:38Z"
concept_id: python-bridge/training/train_gnn/train_thermal_gnn
language: python
---

# train_thermal_gnn

Trains the GNN surrogate model until thermal error converges below tolerance.

## Signature

```python
def train_thermal_gnn(epochs: int = 100, lr: float = 0.001, tolerance_c: float = 2.0, output_path: str = 'crates/models/thermal_gnn.pt') -> bool
```

## Docstring

Trains the GNN surrogate model until thermal error converges below tolerance.

## Parameters

| Name | Type | Default |
|------|------|---------|
| `epochs` | `int` | `100` |

| `lr` | `float` | `0.001` |

| `tolerance_c` | `float` | `2.0` |

| `output_path` | `str` | `'crates/models/thermal_gnn.pt'` |

## Returns
`bool`

## Source
Lines 139–188 in `python-bridge/training/train_gnn.py`

## Relationships

| Type | Target |
|------|--------|
| related | [train_gnn](/python-bridge/training/train_gnn.md) |
| calls | [generate_synthetic_mesh_data](/python-bridge/training/train_gnn/generate_synthetic_mesh_data.md) |
| called_by | [main](/python-bridge/training/train_gnn/main.md) |
