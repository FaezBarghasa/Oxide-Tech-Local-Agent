---
okf_version: "0.2"
type: Function
title: compute_compiler_reward
description: Evaluates generated Rust code using target cargo check.
resource: python-bridge/training/train_rust_model.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-22T11:51:29Z"
concept_id: python-bridge/training/train_rust_model/compute_compiler_reward
language: python
---

# compute_compiler_reward

Evaluates generated Rust code using target cargo check.

## Signature

```python
def compute_compiler_reward(code_snippet: str, target_platform: str = 'thumbv7em-none-eabihf') -> float
```

## Docstring

Evaluates generated Rust code using target cargo check.
Returns:
    1.0 if compile succeeds with no warnings,
    0.5 if compile succeeds with warnings,
    0.0 if compile fails.

## Parameters

| Name | Type | Default |
|------|------|---------|
| `code_snippet` | `str` | `—` |

| `target_platform` | `str` | `'thumbv7em-none-eabihf'` |

## Returns
`float`

## Source
Lines 7–30 in `python-bridge/training/train_rust_model.py`

## Relationships

| Type | Target |
|------|--------|
| related | [train_rust_model](/python-bridge/training/train_rust_model.md) |
| called_by | [main](/python-bridge/training/train_rust_model/main.md) |
