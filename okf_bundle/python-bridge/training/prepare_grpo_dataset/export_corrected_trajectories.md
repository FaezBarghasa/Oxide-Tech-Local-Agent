---
okf_version: "0.2"
type: Function
title: export_corrected_trajectories
resource: python-bridge/training/prepare_grpo_dataset.py
tags:
  - "lang:python"
  - "type:Function"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-22T12:02:38Z"
concept_id: python-bridge/training/prepare_grpo_dataset/export_corrected_trajectories
language: python
---

# export_corrected_trajectories

## Signature

```python
async def export_corrected_trajectories()
```

## Source
Lines 28–88 in `python-bridge/training/prepare_grpo_dataset.py`

## Relationships

| Type | Target |
|------|--------|
| related | [prepare_grpo_dataset](/python-bridge/training/prepare_grpo_dataset.md) |
| calls | [calculate_verifier_reward](/python-bridge/training/prepare_grpo_dataset/calculate_verifier_reward.md) |
| called_by | [main](/python-bridge/training/prepare_grpo_dataset/main.md) |
