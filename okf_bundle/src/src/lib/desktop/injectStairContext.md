---
okf_version: "0.2"
type: Function
title: injectStairContext
description: STAIR Code-ToC search + pack results as chat context prefix. Returns injected prefix (empty when disabled/unavailable).
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/injectStairContext
language: typescript
---

# injectStairContext

STAIR Code-ToC search + pack results as chat context prefix. Returns injected prefix (empty when disabled/unavailable).

## Signature

```typescript
injectStairContext(prompt: string, opts: { cwd?: string; budget?: number; limit?: number } = {}): Promise<{ prefix: string; breadcrumbs: string[]; tokens: number }>
```

## Docstring

STAIR Code-ToC search + pack results as chat context prefix. Returns injected prefix (empty when disabled/unavailable).

## Source
Lines 158–171 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [memorySearch](/src/src/lib/desktop/memorySearch.md) |
| calls | [embedText](/src/src/lib/desktop/embedText.md) |
