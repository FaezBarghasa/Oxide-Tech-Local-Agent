---
okf_version: "0.2"
type: Function
title: memorySearch
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/memorySearch
language: typescript
---

# memorySearch

## Signature

```typescript
memorySearch(
    query: string,
    opts: { cwd?: string; stair?: boolean; limit?: number; budget?: number; withGraph?: boolean } = {},
  ): Promise<EmbedResult>
```

## Source
Lines 89–102 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
| called_by | [MemoryTab](/src/src/components/MemoryTab/MemoryTab.md) |
| called_by | [injectStairContext](/src/src/lib/desktop/injectStairContext.md) |
