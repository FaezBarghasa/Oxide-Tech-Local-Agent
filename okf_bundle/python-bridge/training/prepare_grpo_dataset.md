---
okf_version: "0.2"
type: Module
title: prepare_grpo_dataset
description: Prepares GRPO RLVR rollouts from SurrealDB agent trajectories
resource: python-bridge/training/prepare_grpo_dataset.py
tags:
  - "lang:python"
  - "type:Module"
  - "module:python-bridge"
  - "domain:training"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-22T12:02:38Z"
concept_id: python-bridge/training/prepare_grpo_dataset
language: python
---

# prepare_grpo_dataset

Prepares GRPO RLVR rollouts from SurrealDB agent trajectories

## Docstring

Prepares GRPO RLVR rollouts from SurrealDB agent trajectories
Extracts corrected multi-turn attempts for policy reinforcement

## Relationships

| Type | Target |
|------|--------|
| related | [calculate_verifier_reward](/python-bridge/training/prepare_grpo_dataset/calculate_verifier_reward.md) |
| related | [export_corrected_trajectories](/python-bridge/training/prepare_grpo_dataset/export_corrected_trajectories.md) |
| related | [main](/python-bridge/training/prepare_grpo_dataset/main.md) |
| related | [surrealdb](/_dependencies/pip/surrealdb.md) |
