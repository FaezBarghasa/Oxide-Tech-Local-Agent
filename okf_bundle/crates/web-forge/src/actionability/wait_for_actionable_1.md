---
okf_version: "0.2"
type: Function
title: wait_for_actionable
description: "The 5-Check Playwright Auto-Wait Algorithm:"
resource: crates/web-forge/src/actionability.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:web-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:36:37Z"
concept_id: crates/web-forge/src/actionability/wait_for_actionable_1
language: rust
---

# wait_for_actionable

The 5-Check Playwright Auto-Wait Algorithm:

## Signature

```rust
pub fn wait_for_actionable(
        &self,
        inspector: &I,
        selector: &str,
        config: &ActionabilityConfig,
    ) -> Result<ElementLayoutState, WebForgeError>
```

## Type Parameters

- `I: LayoutInspector`

## Visibility

- `pub`

## Docstring

The 5-Check Playwright Auto-Wait Algorithm:
1. Attached: Element exists in DOM
2. Visible: Non-empty box, display != none, visibility != hidden, opacity > 0
3. Stable: Dimensions and position remain unchanged across consecutive samples
4. Enabled: Not disabled attribute or aria-disabled
5. Uncovered: Hit test at center point returns the element itself (no overlay/spinner)

## Source
Lines 140–241 in `crates/web-forge/src/actionability.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [actionability](/crates/web-forge/src/actionability.md) |
