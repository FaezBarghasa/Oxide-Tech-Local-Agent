# moe_router

## Classs

- [FusedMoeRouterOp](FusedMoeRouterOp.md) — Fused GPU/SIMD Mixture-of-Experts Router kernel for Gemma-4 and sparse MoE architectures.
- [MoeRoutingPlan](MoeRoutingPlan.md) — Output of the Fused MoE Router kernel containing routing indices and normalized expert weights.

## Functions

- [default](default.md)
- [default](default_1.md)
- [new](new.md)
- [new](new_1.md)
- [route_tokens](route_tokens.md) — Perform fused Top-K selection, Softmax normalization, and token-to-expert dispatch routing.
- [route_tokens](route_tokens_1.md) — Perform fused Top-K selection, Softmax normalization, and token-to-expert dispatch routing.
- [test_fused_moe_router_top2_selection](test_fused_moe_router_top2_selection.md) — [test]
- [test_ornith_256_expert_top8_routing](test_ornith_256_expert_top8_routing.md) — [test]
