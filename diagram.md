# Oxide-Tech Local Agent OS: System Diagrams

This document contains Mermaid diagrams illustrating the control flow, request handling sequence, and AI agent execution loops inside the platform.

---

## 1. Unified System Request Flow

This diagram illustrates how incoming client requests (both TCP and UDP) are resolved by the dual-protocol gateway and executed inside sandboxes:

```mermaid
sequenceDiagram
    autonumber
    actor Client
    participant Actix as Actix Gateway (TCP:8080)
    participant Quinn as Quinn H3 (UDP:8080)
    participant Core as Rust Core (Services)
    participant DB as SurrealDB v3
    participant Qdrant as Qdrant Vector DB
    participant Docker as Docker Sandbox

    alt TCP Connection (HTTP/1.1 or HTTP/2)
        Client->>Actix: Request (e.g. POST /api/cargo/check)
        Actix->>Core: Decoupled service function call
    else UDP Connection (HTTP/3 - QUIC)
        Client->>Quinn: QUIC Frame (POST /api/cargo/check)
        Quinn->>Core: Decoupled service function call
    end

    alt Database lookup required
        Core->>DB: Query symbol graphs or workspace context
        DB-->>Core: RecordId / SurrealValue data
    else Semantic search required
        Core->>Qdrant: Hybrid dense/sparse search vector
        Qdrant-->>Core: Payload matching datasheet properties
    end

    alt Code or script execution
        Core->>Docker: Execute script inside container limits
        Docker->>Docker: Run cargo / python script
        Docker-->>Core: Exit code, stdout, stderr
    end

    alt TCP Response
        Core-->>Actix: Execution result JSON
        Actix->>Client: 200 OK + [Alt-Svc: h3=":8080"; ma=86400]
    else HTTP/3 Response
        Core-->>Quinn: Execution result JSON
        Quinn->>Client: HTTP/3 Frame (200 OK)
    end
```

---

## 2. Multi-Agent AI Code Generation Loop (Auto-Healing)

The code generation system uses a multi-agent hierarchy consisting of a **Chief Planner**, an **Execution Agent**, and a **Verification Agent** running in an auto-healing feedback loop:

```mermaid
sequenceDiagram
    autonumber
    actor User
    participant Gateway as API Gateway
    participant Chief as Chief Planner Agent
    participant DB as SurrealDB v3
    participant Editor as Execution Agent
    participant Sandbox as Docker Sandbox
    participant Verifier as Verification Agent

    User->>Gateway: Trigger Code Gen (POST /api/agent/generate)
    Gateway->>DB: Get Workspace context (Parsed symbols)
    DB-->>Gateway: AST context tree
    Gateway->>Chief: Prompt + Workspace context
    Note over Chief: Analyzes dependencies & files
    Chief-->>Gateway: Return Execution Plan (JSON)
    
    loop Execute Plan Steps
        Gateway->>Editor: Execution Plan Steps
        Note over Editor: Formulates write_file / apply_diff edits
        Editor-->>Gateway: Return Tool Calls JSON
        Gateway->>Gateway: Apply file edits to workspace
    end

    loop Auto-Healing Verification Loop (Max 5 attempts)
        Gateway->>Sandbox: Execute "cargo check" in Sandbox
        Sandbox-->>Gateway: Result (Exit Code & Diagnostics)
        
        alt Exit Code == 0 (Success)
            Gateway-->>User: Return status: PASSED (200 OK)
        else Exit Code != 0 (Failure)
            Gateway->>Verifier: Send compiler stderr output
            Note over Verifier: Analyzes error diagnostics
            Verifier-->>Gateway: Return specific Correction Mandate
            Gateway->>Editor: Send Correction Mandate + Plan
            Note over Editor: Refines code diff
            Editor-->>Gateway: Return modified Tool Calls JSON
            Gateway->>Gateway: Apply corrected edits
        end
    end

    alt Healing failed after 5 retries
        Gateway-->>User: Return status: FAILED + compilation stderr
    end
```

---

## 3. Real-time Log Streaming via WebSockets

```mermaid
sequenceDiagram
    autonumber
    actor UI as Client Web App
    participant GW as API Gateway
    participant Broadcaster as WsBroadcaster (Arc)
    participant Task as Async Sandbox Process

    UI->>GW: WS Connection (/ws/compilation)
    GW->>Broadcaster: Subscribe to compilation channel
    GW-->>UI: Upgrade connection (101 Switching Protocols)

    loop Compilation In Progress
        Task->>Broadcaster: Send stdout/stderr line
        Broadcaster->>GW: Broadcast stream message
        GW->>UI: WebSocket Text Frame (log output)
    end

    Task-->>Broadcaster: Compilation completes / terminates
    GW->>UI: Close Frame
```
