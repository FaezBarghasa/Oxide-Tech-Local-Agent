# moe_router

## Classs

- [AdaptiveMoeGatingRouter](AdaptiveMoeGatingRouter.md) — Adaptive MoE Gating Router with dynamic feedback (latency EMA, quality scoring, circuit breaking).
- [ExpertModel](ExpertModel.md) — The target models configured in the Mixture of Experts (MoE) pool
- [ExpertStats](ExpertStats.md) — Operational statistics tracked per expert for adaptive routing
- [MoeGatingRouter](MoeGatingRouter.md) — Mixture of Experts Gating Network
- [MoeRoutingDecision](MoeRoutingDecision.md) — Routing decision output from the MoE Gating Network
- [TaskComplexity](TaskComplexity.md) — Task complexity level determining reasoning compute allocation and `<think>` budget

## Functions

- [classify](classify.md)
- [classify](classify_1.md)
- [default](default.md)
- [default](default_1.md)
- [model_id](model_id.md)
- [model_id](model_id_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [recommended_draft_model](recommended_draft_model.md) — Return the recommended lightweight draft model for speculative decoding pairs
- [recommended_draft_model](recommended_draft_model_1.md) — Return the recommended lightweight draft model for speculative decoding pairs
- [recommended_think_budget](recommended_think_budget.md)
- [recommended_think_budget](recommended_think_budget_1.md)
- [record_feedback](record_feedback.md) — Record runtime execution feedback from an expert
- [record_feedback](record_feedback_1.md) — Record runtime execution feedback from an expert
- [record_verification_outcome](record_verification_outcome.md) — Record verification outcome directly to adjust quality EMA and circuit breaker
- [record_verification_outcome](record_verification_outcome_1.md) — Record verification outcome directly to adjust quality EMA and circuit breaker
- [route](route.md) — Calculate gating logits and select the best expert(s) for a given request
- [route](route_1.md) — Calculate gating logits and select the best expert(s) for a given request
- [route](route_2.md) — Route with adaptive adjustments based on health, latency EMA, and quality
- [route](route_3.md) — Route with adaptive adjustments based on health, latency EMA, and quality
- [specialization](specialization.md)
- [specialization](specialization_1.md)
- [speculative_draft_for](speculative_draft_for.md) — Return the recommended speculative draft model for the chosen primary expert, if any.
- [speculative_draft_for](speculative_draft_for_1.md) — Return the recommended speculative draft model for the chosen primary expert, if any.
