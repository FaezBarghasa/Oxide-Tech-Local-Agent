---
okf_version: "0.2"
type: Function
title: tauriInvoke
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/tauriInvoke
language: typescript
---

# tauriInvoke

## Signature

```typescript
function tauriInvoke(cmd: string, args: Record<string, unknown>): Promise<T>
```

## Type Parameters

- `T`

## Source
Lines 30–33 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| called_by | [configRead](/src/src/lib/desktop/configRead.md) |
| called_by | [configSave](/src/src/lib/desktop/configSave.md) |
| called_by | [doctorInstallUdevRules](/src/src/lib/desktop/doctorInstallUdevRules.md) |
| called_by | [doctorRunDiagnostics](/src/src/lib/desktop/doctorRunDiagnostics.md) |
| called_by | [gatewayDaemonRestart](/src/src/lib/desktop/gatewayDaemonRestart.md) |
| called_by | [gatewayDaemonStart](/src/src/lib/desktop/gatewayDaemonStart.md) |
| called_by | [gatewayStatus](/src/src/lib/desktop/gatewayStatus.md) |
| called_by | [getEngineMatrixStatus](/src/src/lib/desktop/getEngineMatrixStatus.md) |
| called_by | [getSystemTelemetry](/src/src/lib/desktop/getSystemTelemetry.md) |
| called_by | [getTieredCacheMetrics](/src/src/lib/desktop/getTieredCacheMetrics.md) |
| called_by | [hardwareFlashFirmware](/src/src/lib/desktop/hardwareFlashFirmware.md) |
| called_by | [hardwareGetChipInfo](/src/src/lib/desktop/hardwareGetChipInfo.md) |
| called_by | [hardwareListProbes](/src/src/lib/desktop/hardwareListProbes.md) |
| called_by | [memoryConflicts](/src/src/lib/desktop/memoryConflicts.md) |
| called_by | [memoryContext](/src/src/lib/desktop/memoryContext.md) |
| called_by | [memoryEnv](/src/src/lib/desktop/memoryEnv.md) |
| called_by | [memoryExplain](/src/src/lib/desktop/memoryExplain.md) |
| called_by | [memoryIndex](/src/src/lib/desktop/memoryIndex.md) |
| called_by | [memoryInit](/src/src/lib/desktop/memoryInit.md) |
| called_by | [memoryRecall](/src/src/lib/desktop/memoryRecall.md) |
| called_by | [memoryRemember](/src/src/lib/desktop/memoryRemember.md) |
| called_by | [memorySearch](/src/src/lib/desktop/memorySearch.md) |
| called_by | [memoryStatus](/src/src/lib/desktop/memoryStatus.md) |
| called_by | [modelListAvailable](/src/src/lib/desktop/modelListAvailable.md) |
| called_by | [modelRunPrompt](/src/src/lib/desktop/modelRunPrompt.md) |
| called_by | [reforgeAnalyzeFile](/src/src/lib/desktop/reforgeAnalyzeFile.md) |
| called_by | [scanLocalGgufModels](/src/src/lib/desktop/scanLocalGgufModels.md) |
| called_by | [verifierExportEvidence](/src/src/lib/desktop/verifierExportEvidence.md) |
| called_by | [verifierRunSuite](/src/src/lib/desktop/verifierRunSuite.md) |
