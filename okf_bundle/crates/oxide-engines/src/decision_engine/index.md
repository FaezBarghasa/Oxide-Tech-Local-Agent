# decision_engine

## Classs

- [BrierScoreLoss](BrierScoreLoss.md) — Brier Score / Proper Scoring Rule Loss for Calibrated Confidence
- [CandidateVectorCache](CandidateVectorCache.md) — Contrastive candidate cache: pre-computed normalized embeddings for fast dot-product selection
- [DecisionDevice](DecisionDevice.md) — Execution device identifier for local backend compute
- [DecisionEngine](DecisionEngine.md) — Thread-safe client handle. Can be cloned across OS threads or Rayon tasks.
- [DecisionInput](DecisionInput.md) — [derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
- [DecisionOutput](DecisionOutput.md) — [derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
- [DecisionSpreadResult](DecisionSpreadResult.md) — Structured candidate ranking and speculative spread evaluation result
- [FastKanDecisionHead](FastKanDecisionHead.md) — 2-layer MLP / Fast-KAN Pooled State Decision Head
- [InferenceJob](InferenceJob.md) — Internal job passing the task and a one-time response channel
- [SpeculativeCascadeConfig](SpeculativeCascadeConfig.md) — [derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

## Functions

- [compute](compute.md) — Compute Brier score: 1/N * sum((predicted_prob - actual_binary_outcome)^2)
- [compute](compute_1.md) — Compute Brier score: 1/N * sum((predicted_prob - actual_binary_outcome)^2)
- [decide](decide.md) — Synchronous, blocking call. Thread-safe and re-entrant.
- [decide](decide_1.md) — Synchronous, blocking call. Thread-safe and re-entrant.
- [decide_with_cascade](decide_with_cascade.md) — Synchronous decision call with explicit speculative cascading policy
- [decide_with_cascade](decide_with_cascade_1.md) — Synchronous decision call with explicit speculative cascading policy
- [default](default.md)
- [default](default_1.md)
- [forward](forward.md) — Forward pass through pooled latent state
- [forward](forward_1.md) — Forward pass through pooled latent state
- [insert](insert.md)
- [insert](insert_1.md)
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [new](new_4.md)
- [new](new_5.md)
- [new_with_cascade](new_with_cascade.md)
- [new_with_cascade](new_with_cascade_1.md)
- [process_batch](process_batch.md)
- [run_worker_loop](run_worker_loop.md) — --- Dedicated Worker Loop with Dynamic Micro-Batching ---
- [score_candidates_ranked](score_candidates_ranked.md) — Ranks all candidate options with calibrated probabilities
- [score_candidates_ranked](score_candidates_ranked_1.md) — Ranks all candidate options with calibrated probabilities
- [score_state](score_state.md) — Dense dot-product similarity lookup against state embedding
- [score_state](score_state_1.md) — Dense dot-product similarity lookup against state embedding
- [score_state_with_spread](score_state_with_spread.md) — Score state and evaluate speculative cascade thresholds
- [score_state_with_spread](score_state_with_spread_1.md) — Score state and evaluate speculative cascade thresholds
- [test_brier_score_loss_calibration](test_brier_score_loss_calibration.md) — [test]
- [test_candidate_vector_cache_scoring](test_candidate_vector_cache_scoring.md) — [test]
- [test_decision_engine_concurrent_batching](test_decision_engine_concurrent_batching.md) — [test]
- [test_fast_kan_decision_head](test_fast_kan_decision_head.md) — [test]
- [test_speculative_cascading_spread_detection](test_speculative_cascading_spread_detection.md) — [test]
