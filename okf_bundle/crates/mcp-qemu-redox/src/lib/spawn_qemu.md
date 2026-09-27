---
okf_version: "0.2"
type: Function
title: spawn_qemu
resource: crates/mcp-qemu-redox/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-qemu-redox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:33Z"
concept_id: crates/mcp-qemu-redox/src/lib/spawn_qemu
language: rust
---

# spawn_qemu

## Signature

```rust
impl QemuRedoxServer { fn spawn_qemu(&self, image_path: &str) -> Result<(), McpError> }
```

## Source
Lines 58–95 in `crates/mcp-qemu-redox/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-qemu-redox/src/lib.md) |
