---
okf_version: "0.2"
type: Function
title: exec
resource: src/src/components/MemoryTab.tsx
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-19T13:37:19Z"
concept_id: src/src/components/MemoryTab/exec_1
language: typescript
---

# exec

## Signature

```typescript
const exec = (label: string, fn: () => Promise<EmbedResult | string>) =>
```

## Source
Lines 48–58 in `src/src/components/MemoryTab.tsx`

## Relationships

| Type | Target |
|------|--------|
| related | [MemoryTab](/src/src/components/MemoryTab.md) |
| calls | [embedText](/src/src/lib/desktop/embedText.md) |
