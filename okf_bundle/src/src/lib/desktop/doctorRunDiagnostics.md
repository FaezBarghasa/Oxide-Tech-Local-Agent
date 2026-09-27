---
okf_version: "0.2"
type: Function
title: doctorRunDiagnostics
description: Doctor Diagnostics
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/doctorRunDiagnostics
language: typescript
---

# doctorRunDiagnostics

Doctor Diagnostics

## Signature

```typescript
doctorRunDiagnostics(): Promise<any>
```

## Docstring

Doctor Diagnostics

## Source
Lines 174–181 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
| called_by | [DoctorTab](/src/src/components/DoctorTab/DoctorTab.md) |
| called_by | [HardwareClusterStatus](/src/src/components/HardwareClusterStatus/HardwareClusterStatus.md) |
| called_by | [StatusBar](/src/src/components/StatusBar/StatusBar.md) |
| called_by | [pollTelemetry](/src/src/components/StatusBar/pollTelemetry.md) |
