---
okf_version: "0.2"
type: Function
title: record_and_check
description: "Record a tool call signature (e.g., \"tool_name:args_hash\")"
resource: crates/optio/src/oscillation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-08-22T11:36:43Z"
concept_id: crates/optio/src/oscillation/record_and_check
language: rust
---

# record_and_check

Record a tool call signature (e.g., "tool_name:args_hash")

## Signature

```rust
impl OscillationDetector { pub fn record_and_check(&mut self, call_sig: &str) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Record a tool call signature (e.g., "tool_name:args_hash")
Returns Ok(()) if safe, or Err(alert_message) if oscillation threshold is exceeded.

## Source
Lines 21–36 in `crates/optio/src/oscillation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [oscillation](/crates/optio/src/oscillation.md) |
