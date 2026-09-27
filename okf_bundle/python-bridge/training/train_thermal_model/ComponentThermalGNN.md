---
okf_version: "0.2"
type: Class
title: ComponentThermalGNN
description: Message-passing neural network for conjugate heat transfer on PCB topologies.
resource: python-bridge/training/train_thermal_model.py
tags:
  - "lang:python"
  - "type:Class"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T16:35:17Z"
concept_id: python-bridge/training/train_thermal_model/ComponentThermalGNN
language: python
---

# ComponentThermalGNN

Message-passing neural network for conjugate heat transfer on PCB topologies.

## Inheritance

- `nn.Module if TORCH_AVAILABLE else object`

## Docstring

Message-passing neural network for conjugate heat transfer on PCB topologies.
Node features: [power_dissipation_watts, pos_x_mm, pos_y_mm, package_area_mm2, r_theta_ja]
Edge features: [copper_distance_mm, copper_width_mm, thermal_conductivity_w_mk]
Target output: [junction_temperature_c, surface_temperature_c]

## Methods

- `__init__`
- `forward`

## Source
Lines 22–72 in `python-bridge/training/train_thermal_model.py`

## Relationships

| Type | Target |
|------|--------|
| related | [train_thermal_model](/python-bridge/training/train_thermal_model.md) |
| related | [__init__](/python-bridge/training/train_thermal_model/init.md) |
| related | [forward](/python-bridge/training/train_thermal_model/forward.md) |
