---
okf_version: "0.2"
type: Function
title: run_build_command
description: Compiles or runs a target command inside a hermetic Docker container
resource: crates/sandbox/src/docker_sandbox.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:sandbox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/sandbox/src/docker_sandbox/run_build_command_1
language: rust
---

# run_build_command

Compiles or runs a target command inside a hermetic Docker container

## Signature

```rust
pub fn run_build_command(&self, build_cmd: &str) -> Result<(bool, String, String)>
```

## Visibility

- `pub`

## Docstring

Compiles or runs a target command inside a hermetic Docker container

## Source
Lines 34–91 in `crates/sandbox/src/docker_sandbox.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [docker_sandbox](/crates/sandbox/src/docker_sandbox.md) |
