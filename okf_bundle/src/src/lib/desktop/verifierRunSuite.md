---
okf_version: "0.2"
type: Function
title: verifierRunSuite
description: Verifier Suite
resource: src/src/lib/desktop.ts
tags:
  - "lang:typescript"
  - "type:Function"
  - "module:src"
  - "domain:src"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T15:22:50Z"
concept_id: src/src/lib/desktop/verifierRunSuite
language: typescript
---

# verifierRunSuite

Verifier Suite

## Signature

```typescript
verifierRunSuite(request: any): Promise<any>
```

## Docstring

Verifier Suite

## Source
Lines 206–220 in `src/src/lib/desktop.ts`

## Relationships

| Type | Target |
|------|--------|
| related | [desktop](/src/src/lib/desktop.md) |
| calls | [isTauriRuntime](/src/src/lib/desktop/isTauriRuntime.md) |
| calls | [tauriInvoke](/src/src/lib/desktop/tauriInvoke.md) |
| called_by | [VerificationTab](/src/src/components/VerificationTab/VerificationTab.md) |
| called_by | [handleRunVerification](/src/src/components/VerificationTab/handleRunVerification.md) |
