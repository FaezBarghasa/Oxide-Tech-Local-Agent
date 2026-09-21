use oxide_core::OxideError;

/// Output of the Fused MoE Router kernel containing routing indices and normalized expert weights.
#[derive(Debug, Clone, PartialEq)]
pub struct MoeRoutingPlan {
    /// Number of tokens processed.
    pub num_tokens: usize,
    /// Selected top-k expert indices per token (shape: [num_tokens, top_k]).
    pub selected_experts: Vec<Vec<usize>>,
    /// Softmax-normalized routing weights per token (shape: [num_tokens, top_k]).
    pub routing_weights: Vec<Vec<f32>>,
    /// Inverted index mapping expert_id -> list of (token_idx, weight) for parallel dispatch.
    pub expert_dispatches: Vec<Vec<(usize, f32)>>,
}

/// Fused GPU/SIMD Mixture-of-Experts Router kernel for Gemma-4 and sparse MoE architectures.
pub struct FusedMoeRouterOp {
    pub kernel_name: String,
    pub num_experts: usize,
    pub top_k: usize,
}

impl Default for FusedMoeRouterOp {
    fn default() -> Self {
        Self {
            kernel_name: "oxide_fused_moe_router".to_string(),
            num_experts: 8,
            top_k: 2, // 2 active experts out of 8 (e.g. Gemma 4 26B-A4B)
        }
    }
}

impl FusedMoeRouterOp {
    pub fn new(num_experts: usize, top_k: usize) -> Self {
        Self {
            kernel_name: "oxide_fused_moe_router".to_string(),
            num_experts,
            top_k,
        }
    }

    /// Perform fused Top-K selection, Softmax normalization, and token-to-expert dispatch routing.
    ///
    /// `router_logits` has dimensions `num_tokens * num_experts`.
    pub fn route_tokens(
        &self,
        router_logits: &[f32],
        num_tokens: usize,
    ) -> Result<MoeRoutingPlan, OxideError> {
        let expected_len = num_tokens * self.num_experts;
        if router_logits.len() < expected_len {
            return Err(OxideError::Engine(format!(
                "Logits buffer underflow: expected {} floats (tokens={}, experts={}), got {}",
                expected_len,
                num_tokens,
                self.num_experts,
                router_logits.len()
            )));
        }

        if self.top_k == 0 || self.top_k > self.num_experts {
            return Err(OxideError::Engine(format!(
                "Invalid top_k: {} for num_experts: {}",
                self.top_k, self.num_experts
            )));
        }

        let mut selected_experts = Vec::with_capacity(num_tokens);
        let mut routing_weights = Vec::with_capacity(num_tokens);
        let mut expert_dispatches: Vec<Vec<(usize, f32)>> =
            vec![Vec::new(); self.num_experts];

        for t in 0..num_tokens {
            let token_offset = t * self.num_experts;
            let logits = &router_logits[token_offset..token_offset + self.num_experts];

            // 1. Extract (expert_id, logit) pairs and sort to find Top-K
            let mut expert_scores: Vec<(usize, f32)> = logits
                .iter()
                .copied()
                .enumerate()
                .collect();

            // Partial sort for Top-K
            expert_scores.sort_unstable_by(|a, b| {
                b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
            });

            let top_experts: Vec<usize> = expert_scores[..self.top_k]
                .iter()
                .map(|&(e_id, _)| e_id)
                .collect();

            let top_logits: Vec<f32> = expert_scores[..self.top_k]
                .iter()
                .map(|&(_, score)| score)
                .collect();

            // 2. Numerically stable Softmax over the Top-K logits
            let max_logit = top_logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let exps: Vec<f32> = top_logits.iter().map(|&x| (x - max_logit).exp()).collect();
            let sum_exp: f32 = exps.iter().sum();
            let weights: Vec<f32> = exps.iter().map(|&x| x / sum_exp).collect();

            // 3. Register token in inverted expert dispatch map
            for (&e_id, &weight) in top_experts.iter().zip(weights.iter()) {
                expert_dispatches[e_id].push((t, weight));
            }

            selected_experts.push(top_experts);
            routing_weights.push(weights);
        }

        Ok(MoeRoutingPlan {
            num_tokens,
            selected_experts,
            routing_weights,
            expert_dispatches,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fused_moe_router_top2_selection() {
        let router = FusedMoeRouterOp::new(8, 2);

        // Token 0: experts 2 and 5 have highest logits
        // Token 1: experts 0 and 7 have highest logits
        let mut logits = vec![0.0f32; 16];
        logits[2] = 5.0;
        logits[5] = 4.0;

        logits[8 + 0] = 6.0;
        logits[8 + 7] = 3.0;

        let plan = router.route_tokens(&logits, 2).unwrap();

        assert_eq!(plan.num_tokens, 2);
        assert_eq!(plan.selected_experts[0], vec![2, 5]);
        assert_eq!(plan.selected_experts[1], vec![0, 7]);

        // Verify softmax sum == 1.0 per token
        let sum_w0: f32 = plan.routing_weights[0].iter().sum();
        let sum_w1: f32 = plan.routing_weights[1].iter().sum();
        assert!((sum_w0 - 1.0).abs() < 1e-5);
        assert!((sum_w1 - 1.0).abs() < 1e-5);

        // Verify inverted dispatch index
        assert_eq!(plan.expert_dispatches[2].len(), 1);
        assert_eq!(plan.expert_dispatches[2][0].0, 0); // Token 0 routed to Expert 2

        assert_eq!(plan.expert_dispatches[0].len(), 1);
        assert_eq!(plan.expert_dispatches[0][0].0, 1); // Token 1 routed to Expert 0
    }

    #[test]
    fn test_ornith_256_expert_top8_routing() {
        // Ornith-1.5: 256 fine-grained experts with Top-8 active routing
        let router = FusedMoeRouterOp::new(256, 8);

        let mut logits = vec![0.0f32; 256];
        // Set 8 active experts with higher logits
        let target_experts = [12, 45, 78, 102, 155, 199, 210, 250];
        for (rank, &e_id) in target_experts.iter().enumerate() {
            logits[e_id] = 10.0 + (rank as f32);
        }

        let plan = router.route_tokens(&logits, 1).unwrap();
        assert_eq!(plan.num_tokens, 1);
        assert_eq!(plan.selected_experts[0].len(), 8);

        for e_id in target_experts {
            assert!(plan.selected_experts[0].contains(&e_id));
        }

        let weight_sum: f32 = plan.routing_weights[0].iter().sum();
        assert!((weight_sum - 1.0).abs() < 1e-4);
    }
}

