---
okf_version: "0.2"
type: Function
title: connect_stdio
description: Spawn a local process (stdio transport) and bind a client to it.
resource: crates/mcp-clients/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-clients"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/mcp-clients/src/lib/connect_stdio_1
language: rust
---

# connect_stdio

Spawn a local process (stdio transport) and bind a client to it.

## Signature

```rust
pub fn connect_stdio(command: &str, args: &[&str]) -> Result<Self>
```

## Visibility

- `pub`

## Docstring

Spawn a local process (stdio transport) and bind a client to it.

## Source
Lines 32–56 in `crates/mcp-clients/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-clients/src/lib.md) |
| calls | [serve](/python-bridge/server/serve.md) |
