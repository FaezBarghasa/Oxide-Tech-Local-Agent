# rl_engine

## Classs

- [DpoLoss](DpoLoss.md) — DPO (Direct Preference Optimization) Loss
- [GrpoEngine](GrpoEngine.md) — GRPO (Group Relative Policy Optimization) Engine for Reasoning RL
- [MultiDomainReward](MultiDomainReward.md) — Multi-domain objective reward weights and evaluation for GRPO
- [OrpoLoss](OrpoLoss.md) — ORPO (Odds Ratio Preference Optimization) Loss
- [PreferenceSample](PreferenceSample.md) — Preference Pair (Chosen vs. Rejected completions)

## Functions

- [compute_group_advantages](compute_group_advantages.md) — Compute normalized group advantages: A_i = (R_i - mean(R)) / (std(R) + eps)
- [compute_group_advantages](compute_group_advantages_1.md) — Compute normalized group advantages: A_i = (R_i - mean(R)) / (std(R) + eps)
- [compute_policy_loss](compute_policy_loss.md) — Compute GRPO surrogate loss for a token sequence with PPO clipping and reference KL penalty
- [compute_policy_loss](compute_policy_loss_1.md) — Compute GRPO surrogate loss for a token sequence with PPO clipping and reference KL penalty
- [default](default.md)
- [default](default_1.md)
- [default](default_2.md)
- [default](default_3.md)
- [default](default_4.md)
- [default](default_5.md)
- [default](default_6.md)
- [default](default_7.md)
- [evaluate](evaluate.md) — Compute composite scalar reward:
- [evaluate](evaluate_1.md) — Compute composite scalar reward:
- [forward](forward.md) — Compute DPO loss from policy and reference log probabilities
- [forward](forward_1.md) — Compute DPO loss from policy and reference log probabilities
- [forward](forward_2.md) — Compute ORPO loss fusing Cross Entropy with Odds-Ratio penalty
- [forward](forward_3.md) — Compute ORPO loss fusing Cross Entropy with Odds-Ratio penalty
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [test_dpo_loss_computation](test_dpo_loss_computation.md) — [test]
- [test_grpo_group_advantage_normalization](test_grpo_group_advantage_normalization.md) — [test]
