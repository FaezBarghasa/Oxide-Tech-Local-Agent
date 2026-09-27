---
okf_version: "0.2"
type: Function
title: memoryContext
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/memoryContext
language: typescript
---

# memoryContext

## Signature

```typescript
memoryContext(task: string, opts: { cwd?: string; budget?: number } = {}): Promise<EmbedResult>
```

## Source
Lines 134–141 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
| called_by | [MemoryTab](/src/src/components/MemoryTab/MemoryTab.md) |
