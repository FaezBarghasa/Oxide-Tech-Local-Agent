# model_ipc

## Classs

- [DiscoveredGgufModel](DiscoveredGgufModel.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [EngineStatusEntry](EngineStatusEntry.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ModelInfo](ModelInfo.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [ModelListResponse](ModelListResponse.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [RunPromptRequest](RunPromptRequest.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [RunPromptResponse](RunPromptResponse.md) — [derive(Debug, Clone, Serialize, Deserialize)]
- [TieredCacheMetrics](TieredCacheMetrics.md) — [derive(Debug, Clone, Serialize, Deserialize)]

## Functions

- [format_bytes](format_bytes.md)
- [get_engine_matrix_status](get_engine_matrix_status.md) — [tauri::command]
- [get_tiered_cache_metrics](get_tiered_cache_metrics.md) — [tauri::command]
- [inspect_gguf_file](inspect_gguf_file.md)
- [model_list_available](model_list_available.md) — [tauri::command]
- [model_run_prompt](model_run_prompt.md) — [tauri::command]
- [probe_tcp_port](probe_tcp_port.md)
- [query_ollama_models](query_ollama_models.md) — Query local Ollama API for installed models
- [query_sglang_models](query_sglang_models.md) — Query local SGLang / vLLM API for served models
- [scan_default_local_gguf_models](scan_default_local_gguf_models.md) — Scan disk for .gguf model files
- [scan_local_gguf_models](scan_local_gguf_models.md) — [tauri::command]
