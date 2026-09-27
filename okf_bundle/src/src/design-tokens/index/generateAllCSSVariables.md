---
okf_version: "0.2"
type: Function
title: generateAllCSSVariables
resource: src/src/design-tokens/index.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T08:34:20Z"
concept_id: src/src/design-tokens/index/generateAllCSSVariables
language: typescript
---

# generateAllCSSVariables

## Signature

```typescript
function generateAllCSSVariables(mode: ThemeMode = 'dark'): Record<string, string>
```

## Source
Lines 29–39 in `src/src/design-tokens/index.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [design-tokens](/src/src/design-tokens/index.md) |
| calls | [generateSpacingCSSVariables](/src/src/design-tokens/spacing/generateSpacingCSSVariables.md) |
| calls | [generateTypographyCSSVariables](/src/src/design-tokens/typography/generateTypographyCSSVariables.md) |
| calls | [generateRadiiCSSVariables](/src/src/design-tokens/radii/generateRadiiCSSVariables.md) |
| calls | [generateShadowsCSSVariables](/src/src/design-tokens/shadows/generateShadowsCSSVariables.md) |
| calls | [generateTransitionsCSSVariables](/src/src/design-tokens/transitions/generateTransitionsCSSVariables.md) |
| calls | [generateZIndexCSSVariables](/src/src/design-tokens/z-index/generateZIndexCSSVariables.md) |
| called_by | [createThemeStyleElement](/src/src/design-tokens/index/createThemeStyleElement.md) |
| called_by | [injectCSSVariables](/src/src/design-tokens/index/injectCSSVariables.md) |
