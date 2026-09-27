---
okf_version: "0.2"
type: Function
title: chat_completions
resource: crates/oxide-gateway/src/routes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:38:18Z"
concept_id: crates/oxide-gateway/src/routes/chat_completions
language: rust
---

# chat_completions

## Signature

```rust
pub fn chat_completions(
    state: web::Data<Arc<AppState>>,
    req: web::Json<ChatCompletionRequest>,
) -> impl Responder
```

## Visibility

- `pub`

## Source
Lines 68–187 in `crates/oxide-gateway/src/routes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routes](/crates/oxide-gateway/src/routes.md) |
