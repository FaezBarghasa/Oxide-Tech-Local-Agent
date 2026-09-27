---
okf_version: "0.2"
type: Module
title: grammar
description: "# Grammar-Constrained Context-Free Decoding (GBNF & Schema Logit Masking)"
resource: crates/oxide-engines/src/grammar.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:09:50Z"
concept_id: crates/oxide-engines/src/grammar
language: rust
---

# grammar

# Grammar-Constrained Context-Free Decoding (GBNF & Schema Logit Masking)

## Docstring

# Grammar-Constrained Context-Free Decoding (GBNF & Schema Logit Masking)

Enforces 100% JSON schema conformity and deterministic grammar constraints
during token generation, eliminating parsing failures.

## Relationships

| Type | Target |
|------|--------|
| related | [GrammarRule](/crates/oxide-engines/src/grammar/GrammarRule.md) |
| related | [GbnfCompiler](/crates/oxide-engines/src/grammar/GbnfCompiler.md) |
| related | [new](/crates/oxide-engines/src/grammar/new.md) |
| related | [add_rule](/crates/oxide-engines/src/grammar/add_rule.md) |
| related | [compile_json_schema](/crates/oxide-engines/src/grammar/compile_json_schema.md) |
| related | [filter_valid_tokens](/crates/oxide-engines/src/grammar/filter_valid_tokens.md) |
| related | [is_partially_valid](/crates/oxide-engines/src/grammar/is_partially_valid.md) |
| related | [mask_logits](/crates/oxide-engines/src/grammar/mask_logits.md) |
| related | [new](/crates/oxide-engines/src/grammar/new.md) |
| related | [add_rule](/crates/oxide-engines/src/grammar/add_rule.md) |
| related | [compile_json_schema](/crates/oxide-engines/src/grammar/compile_json_schema.md) |
| related | [filter_valid_tokens](/crates/oxide-engines/src/grammar/filter_valid_tokens.md) |
| related | [is_partially_valid](/crates/oxide-engines/src/grammar/is_partially_valid.md) |
| related | [mask_logits](/crates/oxide-engines/src/grammar/mask_logits.md) |
| related | [test_gbnf_json_schema_compilation](/crates/oxide-engines/src/grammar/test_gbnf_json_schema_compilation.md) |
| related | [test_logit_masking](/crates/oxide-engines/src/grammar/test_logit_masking.md) |
