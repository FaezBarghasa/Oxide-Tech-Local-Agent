---
okf_version: "0.2"
type: Function
title: detect
resource: crates/forge-rust/src/detector.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:forge-rust"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/forge-rust/src/detector/detect
language: rust
---

# detect

## Signature

```rust
impl LanguageDetector { pub fn detect(source: &str, file_path: Option<&str>) -> SourceLanguage }
```

## Visibility

- `pub`

## Source
Lines 37–44 in `crates/forge-rust/src/detector.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [detector](/crates/forge-rust/src/detector.md) |
