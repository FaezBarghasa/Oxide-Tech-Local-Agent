---
okf_version: "0.2"
type: Function
title: mock_request
resource: crates/router/tests/moe_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/tests/moe_test/mock_request
language: rust
---

# mock_request

## Signature

```rust
fn mock_request(task_type: TaskType, prompt: &str) -> InferenceRequest
```

## Source
Lines 5–17 in `crates/router/tests/moe_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [moe_test](/crates/router/tests/moe_test.md) |
| called_by | [test_moe_routing_gemma_moe_fast_triage_and_training](/crates/router/tests/moe_test/test_moe_routing_gemma_moe_fast_triage_and_training.md) |
| called_by | [test_moe_routing_llm4decompile_binary_analysis](/crates/router/tests/moe_test/test_moe_routing_llm4decompile_binary_analysis.md) |
| called_by | [test_moe_routing_ornith_embedded_and_architecture](/crates/router/tests/moe_test/test_moe_routing_ornith_embedded_and_architecture.md) |
| called_by | [test_moe_routing_qwen_generalist_syntax](/crates/router/tests/moe_test/test_moe_routing_qwen_generalist_syntax.md) |
| called_by | [test_moe_routing_spark_edge_microcontroller](/crates/router/tests/moe_test/test_moe_routing_spark_edge_microcontroller.md) |
| called_by | [test_moe_routing_turbo_fc_fusion_code_completion](/crates/router/tests/moe_test/test_moe_routing_turbo_fc_fusion_code_completion.md) |
