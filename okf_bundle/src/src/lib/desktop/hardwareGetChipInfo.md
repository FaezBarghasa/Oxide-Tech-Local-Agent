---
okf_version: "0.2"
type: Function
title: hardwareGetChipInfo
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/hardwareGetChipInfo
language: typescript
---

# hardwareGetChipInfo

## Signature

```typescript
hardwareGetChipInfo(deviceIdentifier: string): Promise<any>
```

## Source
Lines 241–251 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
| called_by | [probeRsGetChipInfo](/src/src/lib/desktop/probeRsGetChipInfo.md) |
