---
okf_version: "0.2"
type: Module
title: health
resource: crates/api/src/routes/health.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/health
language: rust
---

# health

## Relationships

| Type | Target |
|------|--------|
| related | [liveness_probe](/crates/api/src/routes/health/liveness_probe.md) |
| related | [readiness_probe](/crates/api/src/routes/health/readiness_probe.md) |
| related | [metrics_endpoint](/crates/api/src/routes/health/metrics_endpoint.md) |
| related | [serde_json](/_dependencies/cargo/serde_json.md) |
| related | [prometheus](/_dependencies/cargo/prometheus.md) |
