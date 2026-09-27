---
okf_version: "0.2"
type: Function
title: simulate_thermal_distribution
description: "Solve thermal dissipation for a list of components:"
resource: python-bridge/training/train_thermal_model.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T16:35:17Z"
concept_id: python-bridge/training/train_thermal_model/simulate_thermal_distribution
language: python
---

# simulate_thermal_distribution

Solve thermal dissipation for a list of components:

## Signature

```python
def simulate_thermal_distribution(components_data, ambient_temp_c = 25.0)
```

## Docstring

Solve thermal dissipation for a list of components:
components_data: list of dicts with keys:
  - ref_des: str
  - power_watts: float
  - x_mm: float
  - y_mm: float
  - r_theta_ja: float (C/W)
Returns: dict mapping ref_des -> estimated junction temp (C)

## Parameters

| Name | Type | Default |
|------|------|---------|
| `components_data` | `—` | `—` |

| `ambient_temp_c` | `—` | `25.0` |

## Source
Lines 75–106 in `python-bridge/training/train_thermal_model.py`

## Relationships

| Type | Target |
|------|--------|
| related | [train_thermal_model](/python-bridge/training/train_thermal_model.md) |
| called_by | [main](/python-bridge/training/train_thermal_model/main.md) |
