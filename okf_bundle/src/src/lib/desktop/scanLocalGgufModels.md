---
okf_version: "0.2"
type: Function
title: scanLocalGgufModels
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/scanLocalGgufModels
language: typescript
---

# scanLocalGgufModels

## Signature

```typescript
scanLocalGgufModels(customPaths: string[] = []): Promise<DiscoveredGgufModel[]>
```

## Source
Lines 402–405 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
