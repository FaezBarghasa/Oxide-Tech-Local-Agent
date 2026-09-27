---
okf_version: "0.2"
type: Function
title: hardwareFlashFirmware
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/hardwareFlashFirmware
language: typescript
---

# hardwareFlashFirmware

## Signature

```typescript
hardwareFlashFirmware(request: any): Promise<any>
```

## Source
Lines 253–263 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
| called_by | [probeRsFlashFirmware](/src/src/lib/desktop/probeRsFlashFirmware.md) |
