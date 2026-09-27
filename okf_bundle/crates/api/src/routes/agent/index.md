# agent

## Classs

- [EditorResponse](EditorResponse.md) — [derive(Debug, Serialize, Deserialize)]
- [GenerateRequest](GenerateRequest.md) — [derive(Debug, Deserialize, Serialize)]
- [ToolCall](ToolCall.md) — [derive(Debug, Serialize, Deserialize)]
- [ToolCallArgs](ToolCallArgs.md) — [derive(Debug, Serialize, Deserialize)]

## Functions

- [apply_diff_async](apply_diff_async.md)
- [collect_rs_files](collect_rs_files.md) — ── Filesystem helpers ────────────────────────────────────────────────────────
- [handle_agent_generate](handle_agent_generate.md) — [post("/api/agent/generate")]
- [handle_agent_stream](handle_agent_stream.md) — [post("/api/agent/stream")]
- [handle_rag_update](handle_rag_update.md) — [post("/api/rag/update")]
- [handle_status](handle_status.md) — [get("/api/status")]
- [run_agent_generate](run_agent_generate.md) — ── Core generation logic ─────────────────────────────────────────────────────
- [strip_json_fences](strip_json_fences.md)
- [write_file_async](write_file_async.md) — ── File tool helpers (always non-blocking) ───────────────────────────────────
