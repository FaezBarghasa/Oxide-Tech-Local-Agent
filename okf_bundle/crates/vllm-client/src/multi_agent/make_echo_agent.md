---
okf_version: "0.2"
type: Function
title: make_echo_agent
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent/make_echo_agent
language: rust
---

# make_echo_agent

## Signature

```rust
fn make_echo_agent(id: &str, role: AgentRole) -> AgentRecord
```

## Source
Lines 974–986 in `crates/vllm-client/src/multi_agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multi_agent](/crates/vllm-client/src/multi_agent.md) |
| called_by | [test_broadcast_reaches_all](/crates/vllm-client/src/multi_agent/test_broadcast_reaches_all.md) |
| called_by | [test_chain_two_agents](/crates/vllm-client/src/multi_agent/test_chain_two_agents.md) |
| called_by | [test_peer_dialogue_terminates](/crates/vllm-client/src/multi_agent/test_peer_dialogue_terminates.md) |
| called_by | [test_peer_dialogue_with_eval](/crates/vllm-client/src/multi_agent/test_peer_dialogue_with_eval.md) |
| called_by | [test_register_and_list](/crates/vllm-client/src/multi_agent/test_register_and_list.md) |
| called_by | [test_send_and_reply](/crates/vllm-client/src/multi_agent/test_send_and_reply.md) |
| called_by | [test_swv_with_dtx](/crates/vllm-client/src/multi_agent/test_swv_with_dtx.md) |
