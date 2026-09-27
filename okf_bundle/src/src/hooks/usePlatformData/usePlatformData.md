---
okf_version: "0.2"
type: Function
title: usePlatformData
resource: src/src/hooks/usePlatformData.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T14:00:10Z"
concept_id: src/src/hooks/usePlatformData/usePlatformData
language: typescript
---

# usePlatformData

## Signature

```typescript
function usePlatformData({
  ipcCommand,
  ipcPayload,
  wsEventName,
  refreshIntervalMs,
  defaultData,
}: UsePlatformDataOptions<T>)
```

## Type Parameters

- `T`

## Source
Lines 13–61 in `src/src/hooks/usePlatformData.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [usePlatformData](/src/src/hooks/usePlatformData.md) |
| calls | [fetchLiveState](/src/src/hooks/usePlatformData/fetchLiveState.md) |
