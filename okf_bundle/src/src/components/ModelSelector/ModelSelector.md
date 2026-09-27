---
okf_version: "0.2"
type: Function
title: ModelSelector
resource: src/src/components/ModelSelector.tsx
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T09:57:36Z"
concept_id: src/src/components/ModelSelector/ModelSelector
language: typescript
---

# ModelSelector

## Signature

```typescript
const ModelSelector = ({
  selectedModel,
  selectedProvider,
  onModelChange,
  temperature,
  onTemperatureChange,
  maxTokens,
  onMaxTokensChange,
}) =>
```

## Source
Lines 15–217 in `src/src/components/ModelSelector.tsx`

## Relationships

| Type | Target |
|------|--------|
| related | [ModelSelector](/src/src/components/ModelSelector.md) |
| calls | [modelListAvailable](/src/src/lib/desktop/modelListAvailable.md) |
