---
okf_version: "0.2"
type: Function
title: unsafe_audit
resource: crates/mcp-cargo-gatekeeper/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-cargo-gatekeeper"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T11:58:12Z"
concept_id: crates/mcp-cargo-gatekeeper/src/lib/unsafe_audit
language: rust
---

# unsafe_audit

## Signature

```rust
impl CargoGatekeeperServer { fn unsafe_audit(
        &self,
        Parameters(input): Parameters<UnsafeAuditInput>,
    ) -> Result<CallToolResult, McpError> }
```

## Source
Lines 117–185 in `crates/mcp-cargo-gatekeeper/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/mcp-cargo-gatekeeper/src/lib.md) |
