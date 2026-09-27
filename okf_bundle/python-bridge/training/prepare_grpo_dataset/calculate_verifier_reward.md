---
okf_version: "0.2"
type: Function
title: calculate_verifier_reward
description: "Computes scalar GRPO reward for QEMU / KVM / Docker verification:"
resource: python-bridge/training/prepare_grpo_dataset.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-22T12:02:38Z"
concept_id: python-bridge/training/prepare_grpo_dataset/calculate_verifier_reward
language: python
---

# calculate_verifier_reward

Computes scalar GRPO reward for QEMU / KVM / Docker verification:

## Signature

```python
def calculate_verifier_reward(compile_passed: bool, boot_passed: bool, timeout_detected: bool, fault_detected: bool) -> float
```

## Docstring

Computes scalar GRPO reward for QEMU / KVM / Docker verification:
  +1.0 : Clean compile + QEMU/KVM boot passed with 0 errors
  +0.5 : Compile passed, but QEMU boot timed out
  -0.5 : Cortex-M HardFault / Redox Kernel Panic detected
  -1.0 : Docker build failure / Syntax error

## Parameters

| Name | Type | Default |
|------|------|---------|
| `compile_passed` | `bool` | `—` |

| `boot_passed` | `bool` | `—` |

| `timeout_detected` | `bool` | `—` |

| `fault_detected` | `bool` | `—` |

## Returns
`float`

## Source
Lines 10–26 in `python-bridge/training/prepare_grpo_dataset.py`

## Relationships

| Type | Target |
|------|--------|
| related | [prepare_grpo_dataset](/python-bridge/training/prepare_grpo_dataset.md) |
| called_by | [export_corrected_trajectories](/python-bridge/training/prepare_grpo_dataset/export_corrected_trajectories.md) |
