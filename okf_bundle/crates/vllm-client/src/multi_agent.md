---
okf_version: "0.2"
type: Module
title: multi_agent
resource: crates/vllm-client/src/multi_agent.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/multi_agent
language: rust
---

# multi_agent

## Relationships

| Type | Target |
|------|--------|
| related | [AgentRole](/crates/vllm-client/src/multi_agent/AgentRole.md) |
| related | [as_str](/crates/vllm-client/src/multi_agent/as_str.md) |
| related | [as_str](/crates/vllm-client/src/multi_agent/as_str.md) |
| related | [PeerDialogueResult](/crates/vllm-client/src/multi_agent/PeerDialogueResult.md) |
| related | [AgentMessage](/crates/vllm-client/src/multi_agent/AgentMessage.md) |
| related | [new](/crates/vllm-client/src/multi_agent/new.md) |
| related | [with_thread](/crates/vllm-client/src/multi_agent/with_thread.md) |
| related | [with_dtx](/crates/vllm-client/src/multi_agent/with_dtx.md) |
| related | [with_payload](/crates/vllm-client/src/multi_agent/with_payload.md) |
| related | [new](/crates/vllm-client/src/multi_agent/new.md) |
| related | [with_thread](/crates/vllm-client/src/multi_agent/with_thread.md) |
| related | [with_dtx](/crates/vllm-client/src/multi_agent/with_dtx.md) |
| related | [with_payload](/crates/vllm-client/src/multi_agent/with_payload.md) |
| related | [AgentRecord](/crates/vllm-client/src/multi_agent/AgentRecord.md) |
| related | [build_loop_runner](/crates/vllm-client/src/multi_agent/build_loop_runner.md) |
| related | [complete](/crates/vllm-client/src/multi_agent/complete.md) |
| related | [build_loop_runner](/crates/vllm-client/src/multi_agent/build_loop_runner.md) |
| related | [complete](/crates/vllm-client/src/multi_agent/complete.md) |
| related | [AgentThread](/crates/vllm-client/src/multi_agent/AgentThread.md) |
| related | [new](/crates/vllm-client/src/multi_agent/new.md) |
| related | [push](/crates/vllm-client/src/multi_agent/push.md) |
| related | [as_chat_history_for](/crates/vllm-client/src/multi_agent/as_chat_history_for.md) |
| related | [new](/crates/vllm-client/src/multi_agent/new.md) |
| related | [push](/crates/vllm-client/src/multi_agent/push.md) |
| related | [as_chat_history_for](/crates/vllm-client/src/multi_agent/as_chat_history_for.md) |
| related | [MultiAgentCoordinator](/crates/vllm-client/src/multi_agent/MultiAgentCoordinator.md) |
| related | [default](/crates/vllm-client/src/multi_agent/default.md) |
| related | [default](/crates/vllm-client/src/multi_agent/default.md) |
| related | [new](/crates/vllm-client/src/multi_agent/new.md) |
| related | [subscribe_events](/crates/vllm-client/src/multi_agent/subscribe_events.md) |
| related | [register](/crates/vllm-client/src/multi_agent/register.md) |
| related | [new_thread](/crates/vllm-client/src/multi_agent/new_thread.md) |
| related | [send](/crates/vllm-client/src/multi_agent/send.md) |
| related | [broadcast](/crates/vllm-client/src/multi_agent/broadcast.md) |
| related | [run_chain](/crates/vllm-client/src/multi_agent/run_chain.md) |
| related | [run_chain_with_dtx](/crates/vllm-client/src/multi_agent/run_chain_with_dtx.md) |
| related | [run_supervisor_worker_verifier](/crates/vllm-client/src/multi_agent/run_supervisor_worker_verifier.md) |
| related | [run_supervisor_worker_verifier_with_dtx](/crates/vllm-client/src/multi_agent/run_supervisor_worker_verifier_with_dtx.md) |
| related | [run_peer_dialogue](/crates/vllm-client/src/multi_agent/run_peer_dialogue.md) |
| related | [run_peer_dialogue_with_eval](/crates/vllm-client/src/multi_agent/run_peer_dialogue_with_eval.md) |
| related | [thread_history](/crates/vllm-client/src/multi_agent/thread_history.md) |
| related | [thread_dtx_messages](/crates/vllm-client/src/multi_agent/thread_dtx_messages.md) |
| related | [list_agents](/crates/vllm-client/src/multi_agent/list_agents.md) |
| related | [new](/crates/vllm-client/src/multi_agent/new.md) |
| related | [subscribe_events](/crates/vllm-client/src/multi_agent/subscribe_events.md) |
| related | [register](/crates/vllm-client/src/multi_agent/register.md) |
| related | [new_thread](/crates/vllm-client/src/multi_agent/new_thread.md) |
| related | [send](/crates/vllm-client/src/multi_agent/send.md) |
| related | [broadcast](/crates/vllm-client/src/multi_agent/broadcast.md) |
| related | [run_chain](/crates/vllm-client/src/multi_agent/run_chain.md) |
| related | [run_chain_with_dtx](/crates/vllm-client/src/multi_agent/run_chain_with_dtx.md) |
| related | [run_supervisor_worker_verifier](/crates/vllm-client/src/multi_agent/run_supervisor_worker_verifier.md) |
| related | [run_supervisor_worker_verifier_with_dtx](/crates/vllm-client/src/multi_agent/run_supervisor_worker_verifier_with_dtx.md) |
| related | [run_peer_dialogue](/crates/vllm-client/src/multi_agent/run_peer_dialogue.md) |
| related | [run_peer_dialogue_with_eval](/crates/vllm-client/src/multi_agent/run_peer_dialogue_with_eval.md) |
| related | [thread_history](/crates/vllm-client/src/multi_agent/thread_history.md) |
| related | [thread_dtx_messages](/crates/vllm-client/src/multi_agent/thread_dtx_messages.md) |
| related | [list_agents](/crates/vllm-client/src/multi_agent/list_agents.md) |
| related | [AgentBuilder](/crates/vllm-client/src/multi_agent/AgentBuilder.md) |
| related | [new](/crates/vllm-client/src/multi_agent/new.md) |
| related | [system_prompt](/crates/vllm-client/src/multi_agent/system_prompt.md) |
| related | [provider](/crates/vllm-client/src/multi_agent/provider.md) |
| related | [tool_executor](/crates/vllm-client/src/multi_agent/tool_executor.md) |
| related | [tools](/crates/vllm-client/src/multi_agent/tools.md) |
| related | [max_turns](/crates/vllm-client/src/multi_agent/max_turns.md) |
| related | [build](/crates/vllm-client/src/multi_agent/build.md) |
| related | [new](/crates/vllm-client/src/multi_agent/new.md) |
| related | [system_prompt](/crates/vllm-client/src/multi_agent/system_prompt.md) |
| related | [provider](/crates/vllm-client/src/multi_agent/provider.md) |
| related | [tool_executor](/crates/vllm-client/src/multi_agent/tool_executor.md) |
| related | [tools](/crates/vllm-client/src/multi_agent/tools.md) |
| related | [max_turns](/crates/vllm-client/src/multi_agent/max_turns.md) |
| related | [build](/crates/vllm-client/src/multi_agent/build.md) |
| related | [EchoProvider](/crates/vllm-client/src/multi_agent/EchoProvider.md) |
| related | [capabilities](/crates/vllm-client/src/multi_agent/capabilities.md) |
| related | [chat_completion](/crates/vllm-client/src/multi_agent/chat_completion.md) |
| related | [stream_chat](/crates/vllm-client/src/multi_agent/stream_chat.md) |
| related | [health](/crates/vllm-client/src/multi_agent/health.md) |
| related | [provider_name](/crates/vllm-client/src/multi_agent/provider_name.md) |
| related | [capabilities](/crates/vllm-client/src/multi_agent/capabilities.md) |
| related | [chat_completion](/crates/vllm-client/src/multi_agent/chat_completion.md) |
| related | [stream_chat](/crates/vllm-client/src/multi_agent/stream_chat.md) |
| related | [health](/crates/vllm-client/src/multi_agent/health.md) |
| related | [provider_name](/crates/vllm-client/src/multi_agent/provider_name.md) |
| related | [make_echo_agent](/crates/vllm-client/src/multi_agent/make_echo_agent.md) |
| related | [test_register_and_list](/crates/vllm-client/src/multi_agent/test_register_and_list.md) |
| related | [test_send_and_reply](/crates/vllm-client/src/multi_agent/test_send_and_reply.md) |
| related | [test_chain_two_agents](/crates/vllm-client/src/multi_agent/test_chain_two_agents.md) |
| related | [test_peer_dialogue_terminates](/crates/vllm-client/src/multi_agent/test_peer_dialogue_terminates.md) |
| related | [test_broadcast_reaches_all](/crates/vllm-client/src/multi_agent/test_broadcast_reaches_all.md) |
| related | [test_agent_builder](/crates/vllm-client/src/multi_agent/test_agent_builder.md) |
| related | [test_peer_dialogue_with_eval](/crates/vllm-client/src/multi_agent/test_peer_dialogue_with_eval.md) |
| related | [test_swv_with_dtx](/crates/vllm-client/src/multi_agent/test_swv_with_dtx.md) |
| related | [anyhow](/_dependencies/cargo/anyhow.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
| related | [tracing](/_dependencies/cargo/tracing.md) |
