---
okf_version: "0.2"
type: Function
title: persist_receipt_sync
description: Fsync receipt to disk
resource: crates/oxide-security/src/session_supervisor.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/session_supervisor/persist_receipt_sync_1
language: rust
---

# persist_receipt_sync

Fsync receipt to disk

## Signature

```rust
fn persist_receipt_sync(&self, receipt: &SessionReceipt) -> Result<(), SessionSupervisorError>
```

## Docstring

Fsync receipt to disk

## Source
Lines 143–162 in `crates/oxide-security/src/session_supervisor.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [session_supervisor](/crates/oxide-security/src/session_supervisor.md) |
