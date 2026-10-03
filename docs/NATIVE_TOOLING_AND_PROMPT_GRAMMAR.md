# Native In-Process Tooling & Prompt Grammar Specification

This document specifies the in-process native tool execution framework and the XML prompt grammar implemented in [`crates/oxide-tooling`](file:///home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/crates/oxide-tooling).

---

## 1. NativeTool Trait & Registry Architecture

Traditional agent architectures execute tools over external HTTP endpoints, CLI subshells, or JSON-RPC IPC, incurring significant serialization overhead and process boundary penalties (often $> 50\text{ms}$ per tool invocation). 

Oxide-Tech introduces **In-Process Native Tool Calling** (`crates/oxide-tooling/src/native_tools.rs`), allowing the reasoning loop to invoke registered Rust tools directly in memory with sub-millisecond dispatch:

```rust
#[async_trait::async_trait]
pub trait NativeTool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, params: serde_json::Value) -> Result<serde_json::Value, ToolError>;
}
```

### Thread-Safe Registry (`NativeToolRegistry`):
The `NativeToolRegistry` provides thread-safe registration and dispatch across asynchronous Tokio tasks:

```rust
let mut registry = NativeToolRegistry::new();
registry.register(Arc::new(StairSearchTool));
registry.register(Arc::new(MemoryRecallTool));
registry.register(Arc::new(HardwareProbeTool));
registry.register(Arc::new(PtxDecompileTool));

// Execute in-process:
let output = registry.execute("stair_search", json!({"query": "ternary_gemm"})).await?;
```

---

## 2. Built-In Native Tool Catalog

| Tool Name | Rust Struct | Purpose | Parameters |
| :--- | :--- | :--- | :--- |
| `stair_search` | `StairSearchTool` | Hierarchical Code-ToC leaf search via `oxide-embed` with enclosing breadcrumbs. | `{"query": string, "max_results": optional int}` |
| `memory_recall` | `MemoryRecallTool` | Recalls architectural decisions and verified invariants from Memanto. | `{"topic": string}` |
| `hardware_probe` | `HardwareProbeTool` | Scans physical USB/SWD buses for connected microcontrollers via `probe-rs`. | `{}` |
| `ptx_decompile` | `PtxDecompileTool` | Decompiles CUDA PTX or SASS assembly into idiomatic safe-Rust kernels. | `{"path": string}` |

---

## 3. Ornith XML Prompt Grammar (`OrnithPromptFormatter`)

To prevent token bleeding and enforce structured tool usage across offline models (such as `Ornith-1.5` or `Qwen2.5-Coder`), tool definitions and invocations follow a deterministic XML schema:

### A. Prompt Tool Injection Format:
The agent orchestrator formats active tools into a `<tools>` block within the model's system prompt:

```xml
You have access to the following native in-process tools:

<tools>
  <tool>
    <name>stair_search</name>
    <description>Hierarchical Code-ToC leaf search via oxide-embed with enclosing breadcrumbs.</description>
    <parameters>
      {"type":"object","properties":{"query":{"type":"string"}},"required":["query"]}
    </parameters>
  </tool>
  <tool>
    <name>memory_recall</name>
    <description>Recalls architectural decisions and verified invariants from Memanto memory fabric.</description>
    <parameters>
      {"type":"object","properties":{"topic":{"type":"string"}},"required":["topic"]}
    </parameters>
  </tool>
</tools>

To call a tool, respond with a <tool_call> block formatted as:
<tool_call>
{"name": "tool_name", "arguments": {"param1": "value1"}}
</tool_call>
```

### B. Extraction & In-Process Dispatch:
When the model produces a response, `OrnithPromptFormatter::extract_tool_calls` parses the output:

```rust
pub fn extract_tool_calls(response: &str) -> Vec<ParsedToolCall> {
    // Regex matches <tool_call>(.*?)</tool_call> and deserializes JSON payload
}
```

The parsed tool call is dispatched directly through `NativeToolRegistry::execute`, and the formatted JSON result is injected back into the conversation context as:

```xml
<tool_response>
{
  "status": "success",
  "data": { ... }
}
</tool_response>
```

---

## 4. ReAct Loop Execution Invariants

1. **Max ReAct Steps**: The reasoning loop aborts if the model exceeds 8 consecutive tool calls without emitting a final answer.
2. **Oscillation Guard**: If the model invokes the exact same tool with identical parameters twice consecutively, the prompt injector injects an oscillation warning and forces plan reassessment.
3. **Zero Execution Panic**: Any tool failure is safely intercepted and returned as an error payload in `<tool_response>` so the model can self-correct.
