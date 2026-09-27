---
okf_version: "0.2"
type: Function
title: memoryEnv
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/memoryEnv
language: typescript
---

# memoryEnv

## Signature

```typescript
memoryEnv(cwd?: string): Promise<MemoryEnv>
```

## Source
Lines 54–64 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
| called_by | [MemoryTab](/src/src/components/MemoryTab/MemoryTab.md) |
| called_by | [OverviewTab](/src/src/components/OverviewTab/OverviewTab.md) |
| called_by | [loadData](/src/src/components/OverviewTab/loadData.md) |
