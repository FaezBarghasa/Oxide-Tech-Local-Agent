---
okf_version: "0.2"
type: Module
title: desktop
description: "Desktop bridge: Oxide Agent Studio ↔ Tauri backend (`src-tauri`)."
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Module"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop
language: typescript
---

# desktop

Desktop bridge: Oxide Agent Studio ↔ Tauri backend (`src-tauri`).

## Docstring

Desktop bridge: Oxide Agent Studio ↔ Tauri backend (`src-tauri`).
Inside the .deb-installed desktop app every memory operation is a Tauri
command that shells out to the bundled `oxide-embed` sidecar, so the UI
never needs the gateway for memory. When running in a plain browser
(vite dev / express server), calls fall back to the gateway HTTP API.

## Relationships

| Type | Target |
|------|--------|
| related | [EmbedResult](/src/src/lib/desktop/EmbedResult.md) |
| related | [MemoryEnv](/src/src/lib/desktop/MemoryEnv.md) |
| related | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| related | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
| related | [gatewayGet](/src/src/lib/desktop/gatewayGet.md) |
| related | [embedText](/src/src/lib/desktop/embedText.md) |
| related | [memoryEnv](/src/src/lib/desktop/memoryEnv.md) |
| related | [gatewayStatus](/src/src/lib/desktop/gatewayStatus.md) |
| related | [memoryStatus](/src/src/lib/desktop/memoryStatus.md) |
| related | [memoryInit](/src/src/lib/desktop/memoryInit.md) |
| related | [memoryIndex](/src/src/lib/desktop/memoryIndex.md) |
| related | [memorySearch](/src/src/lib/desktop/memorySearch.md) |
| related | [memoryRecall](/src/src/lib/desktop/memoryRecall.md) |
| related | [memoryRemember](/src/src/lib/desktop/memoryRemember.md) |
| related | [memoryContext](/src/src/lib/desktop/memoryContext.md) |
| related | [memoryExplain](/src/src/lib/desktop/memoryExplain.md) |
| related | [memoryConflicts](/src/src/lib/desktop/memoryConflicts.md) |
| related | [injectStairContext](/src/src/lib/desktop/injectStairContext.md) |
| related | [doctorRunDiagnostics](/src/src/lib/desktop/doctorRunDiagnostics.md) |
| related | [doctorInstallUdevRules](/src/src/lib/desktop/doctorInstallUdevRules.md) |
| related | [reforgeAnalyzeFile](/src/src/lib/desktop/reforgeAnalyzeFile.md) |
| related | [verifierRunSuite](/src/src/lib/desktop/verifierRunSuite.md) |
| related | [verifierExportEvidence](/src/src/lib/desktop/verifierExportEvidence.md) |
| related | [hardwareListProbes](/src/src/lib/desktop/hardwareListProbes.md) |
| related | [hardwareGetChipInfo](/src/src/lib/desktop/hardwareGetChipInfo.md) |
| related | [hardwareFlashFirmware](/src/src/lib/desktop/hardwareFlashFirmware.md) |
| related | [probeRsListDevices](/src/src/lib/desktop/probeRsListDevices.md) |
| related | [probeRsGetChipInfo](/src/src/lib/desktop/probeRsGetChipInfo.md) |
| related | [probeRsFlashFirmware](/src/src/lib/desktop/probeRsFlashFirmware.md) |
| related | [gatewayDaemonStart](/src/src/lib/desktop/gatewayDaemonStart.md) |
| related | [gatewayDaemonStop](/src/src/lib/desktop/gatewayDaemonStop.md) |
| related | [gatewayDaemonRestart](/src/src/lib/desktop/gatewayDaemonRestart.md) |
| related | [gatewayDaemonLogs](/src/src/lib/desktop/gatewayDaemonLogs.md) |
| related | [configRead](/src/src/lib/desktop/configRead.md) |
| related | [configLoad](/src/src/lib/desktop/configLoad.md) |
| related | [configSave](/src/src/lib/desktop/configSave.md) |
| related | [modelListAvailable](/src/src/lib/desktop/modelListAvailable.md) |
| related | [modelRunPrompt](/src/src/lib/desktop/modelRunPrompt.md) |
| related | [getSystemTelemetry](/src/src/lib/desktop/getSystemTelemetry.md) |
| related | [scanLocalGgufModels](/src/src/lib/desktop/scanLocalGgufModels.md) |
| related | [getEngineMatrixStatus](/src/src/lib/desktop/getEngineMatrixStatus.md) |
| related | [getTieredCacheMetrics](/src/src/lib/desktop/getTieredCacheMetrics.md) |
| related | [SystemTelemetryPayload](/src/src/lib/desktop/SystemTelemetryPayload.md) |
| related | [DiscoveredGgufModel](/src/src/lib/desktop/DiscoveredGgufModel.md) |
| related | [EngineStatusEntry](/src/src/lib/desktop/EngineStatusEntry.md) |
| related | [TieredCacheMetrics](/src/src/lib/desktop/TieredCacheMetrics.md) |
| related | [ModelInfo](/src/src/lib/desktop/ModelInfo.md) |
| related | [ModelListResponse](/src/src/lib/desktop/ModelListResponse.md) |
| related | [RunPromptRequest](/src/src/lib/desktop/RunPromptRequest.md) |
| related | [RunPromptResponse](/src/src/lib/desktop/RunPromptResponse.md) |
| related | [usize](/src/src/lib/desktop/usize.md) |
