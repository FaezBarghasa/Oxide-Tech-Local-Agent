---
okf_version: "0.2"
type: Function
title: createThemeStyleElement
resource: src/src/design-tokens/index.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T08:34:20Z"
concept_id: src/src/design-tokens/index/createThemeStyleElement
language: typescript
---

# createThemeStyleElement

## Signature

```typescript
function createThemeStyleElement(mode: ThemeMode = 'dark', id = 'design-tokens'): HTMLStyleElement
```

## Source
Lines 48–67 in `src/src/design-tokens/index.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [design-tokens](/src/src/design-tokens/index.md) |
| calls | [generateAllCSSVariables](/src/src/design-tokens/index/generateAllCSSVariables.md) |
| called_by | [initializeTheme](/src/src/design-tokens/index/initializeTheme.md) |
| called_by | [setTheme](/src/src/design-tokens/index/setTheme.md) |
