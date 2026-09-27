---
okf_version: "0.2"
type: Function
title: subscribeToThemeChanges
resource: src/src/design-tokens/index.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T08:34:20Z"
concept_id: src/src/design-tokens/index/subscribeToThemeChanges
language: typescript
---

# subscribeToThemeChanges

## Signature

```typescript
function subscribeToThemeChanges(callback: (mode: ThemeMode) => void): () => void
```

## Source
Lines 104–126 in `src/src/design-tokens/index.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [design-tokens](/src/src/design-tokens/index.md) |
| calls | [getThemeFromStorage](/src/src/design-tokens/index/getThemeFromStorage.md) |
| called_by | [ThemeProvider](/src/src/providers/ThemeProvider/ThemeProvider.md) |
