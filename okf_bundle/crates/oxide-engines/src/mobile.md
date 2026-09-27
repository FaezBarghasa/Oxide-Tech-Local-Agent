---
okf_version: "0.2"
type: Module
title: mobile
description: "# Mobile FFI & UniFFI Scaffolding"
resource: crates/oxide-engines/src/mobile.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:53:11Z"
concept_id: crates/oxide-engines/src/mobile
language: rust
---

# mobile

# Mobile FFI & UniFFI Scaffolding

## Docstring

# Mobile FFI & UniFFI Scaffolding

Provides zero-overhead, thread-safe C-FFI and UniFFI bindings for embedding
the non-autoregressive `DecisionEngine` directly into iOS (Swift) and Android (Kotlin).

## Relationships

| Type | Target |
|------|--------|
| related | [MobileDecisionEngine](/crates/oxide-engines/src/mobile/MobileDecisionEngine.md) |
| related | [new](/crates/oxide-engines/src/mobile/new.md) |
| related | [decide](/crates/oxide-engines/src/mobile/decide.md) |
| related | [decide_with_cascade](/crates/oxide-engines/src/mobile/decide_with_cascade.md) |
| related | [new](/crates/oxide-engines/src/mobile/new.md) |
| related | [decide](/crates/oxide-engines/src/mobile/decide.md) |
| related | [decide_with_cascade](/crates/oxide-engines/src/mobile/decide_with_cascade.md) |
| related | [oxide_decision_engine_create](/crates/oxide-engines/src/mobile/oxide_decision_engine_create.md) |
| related | [oxide_decision_engine_free](/crates/oxide-engines/src/mobile/oxide_decision_engine_free.md) |
| related | [oxide_decision_engine_decide_json](/crates/oxide-engines/src/mobile/oxide_decision_engine_decide_json.md) |
| related | [oxide_decision_engine_free_string](/crates/oxide-engines/src/mobile/oxide_decision_engine_free_string.md) |
| related | [test_mobile_decision_engine_wrapper](/crates/oxide-engines/src/mobile/test_mobile_decision_engine_wrapper.md) |
| related | [test_c_ffi_json_bridge](/crates/oxide-engines/src/mobile/test_c_ffi_json_bridge.md) |
