use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String, // "local_gguf", "ollama", "sglang", "vllm", "cloud"
    pub size_formatted: String,
    pub path: Option<String>,
    pub is_running: bool,
    pub context_length: usize,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListResponse {
    pub active_model: String,
    pub active_provider: String,
    pub local_gguf_count: usize,
    pub ollama_count: usize,
    pub models: Vec<ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunPromptRequest {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub model: String,
    pub provider: String,
    pub base_url: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stair_context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunPromptResponse {
    pub text: String,
    pub model: String,
    pub provider: String,
    pub tokens_used: Option<usize>,
    pub latency_ms: u64,
    pub error: Option<String>,
}

fn format_bytes(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Scan disk for .gguf model files
fn scan_local_gguf_models() -> Vec<ModelInfo> {
    let mut models = Vec::new();
    let search_dirs = [
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join("models")),
        Some(PathBuf::from("/var/lib/oxide-tech/models")),
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".cache").join("models")),
        Some(PathBuf::from("/tmp/models")),
    ];

    for dir in search_dirs.into_iter().flatten() {
        if !dir.exists() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("gguf") {
                    let file_name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                    models.push(ModelInfo {
                        id: format!("local:{}", file_name),
                        name: file_name.clone(),
                        provider: "local_gguf".to_string(),
                        size_formatted: format_bytes(file_size),
                        path: Some(path.display().to_string()),
                        is_running: false,
                        context_length: 8192,
                        description: format!("Local GGUF in {}", dir.display()),
                    });
                }
            }
        }
    }

    models
}

/// Query local Ollama API for installed models
async fn query_ollama_models(client: &reqwest::Client) -> Vec<ModelInfo> {
    let mut models = Vec::new();
    let url = "http://127.0.0.1:11434/api/tags";

    if let Ok(resp) = client.get(url).send().await {
        if resp.status().is_success() {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(arr) = json.get("models").and_then(|m| m.as_array()) {
                    for item in arr {
                        let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("");
                        let size = item.get("size").and_then(|s| s.as_u64()).unwrap_or(0);
                        let digest = item.get("digest").and_then(|d| d.as_str()).unwrap_or("");
                        let short_digest = if digest.len() > 12 {
                            &digest[..12]
                        } else {
                            digest
                        };

                        if !name.is_empty() {
                            models.push(ModelInfo {
                                id: format!("ollama:{}", name),
                                name: name.to_string(),
                                provider: "ollama".to_string(),
                                size_formatted: format_bytes(size),
                                path: None,
                                is_running: true,
                                context_length: 8192,
                                description: format!("Ollama local model ({})", short_digest),
                            });
                        }
                    }
                }
            }
        }
    }

    models
}

/// Query local SGLang / vLLM API for served models
async fn query_sglang_models(client: &reqwest::Client) -> Vec<ModelInfo> {
    let mut models = Vec::new();
    let sglang_url = "http://127.0.0.1:30000/v1/models";

    if let Ok(resp) = client.get(sglang_url).send().await {
        if resp.status().is_success() {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(arr) = json.get("data").and_then(|m| m.as_array()) {
                    for item in arr {
                        let id = item.get("id").and_then(|n| n.as_str()).unwrap_or("");
                        if !id.is_empty() {
                            models.push(ModelInfo {
                                id: format!("sglang:{}", id),
                                name: id.to_string(),
                                provider: "sglang".to_string(),
                                size_formatted: "GPU VRAM".to_string(),
                                path: None,
                                is_running: true,
                                context_length: 32768,
                                description: "Active SGLang High-Throughput Server".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    models
}

#[tauri::command]
pub async fn model_list_available() -> Result<ModelListResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(1500))
        .build()
        .map_err(|e| e.to_string())?;

    let mut local_ggufs = scan_local_gguf_models();
    let mut ollama_models = query_ollama_models(&client).await;
    let mut sglang_models = query_sglang_models(&client).await;

    let gguf_count = local_ggufs.len();
    let ollama_count = ollama_models.len();

    // Built-in presets for local/cloud inference if user has keys or endpoints
    let standard_presets = vec![
        ModelInfo {
            id: "preset:qwen2.5-coder:7b".to_string(),
            name: "Qwen 2.5 Coder 7B (Recommended)".to_string(),
            provider: "ollama".to_string(),
            size_formatted: "4.7 GB".to_string(),
            path: None,
            is_running: false,
            context_length: 32768,
            description: "Top-tier embedded, systems, and Rust code generation".to_string(),
        },
        ModelInfo {
            id: "preset:deepseek-r1:8b".to_string(),
            name: "DeepSeek R1 8B (Reasoning)".to_string(),
            provider: "ollama".to_string(),
            size_formatted: "4.9 GB".to_string(),
            path: None,
            is_running: false,
            context_length: 16384,
            description: "High-level chain-of-thought planning & verifier synthesis".to_string(),
        },
        ModelInfo {
            id: "preset:llama3.2:3b".to_string(),
            name: "Llama 3.2 3B (Lightweight)".to_string(),
            provider: "ollama".to_string(),
            size_formatted: "2.0 GB".to_string(),
            path: None,
            is_running: false,
            context_length: 8192,
            description: "Fast token generation on CPU / low-VRAM laptops".to_string(),
        },
        ModelInfo {
            id: "cloud:gemini-2.5-flash".to_string(),
            name: "Gemini 2.5 Flash (Cloud)".to_string(),
            provider: "cloud".to_string(),
            size_formatted: "Cloud API".to_string(),
            path: None,
            is_running: true,
            context_length: 1048576,
            description: "Google Gemini API with massive 1M token context".to_string(),
        },
        ModelInfo {
            id: "cloud:groq-llama-3.3-70b".to_string(),
            name: "Groq Llama 3.3 70B (Cloud)".to_string(),
            provider: "cloud".to_string(),
            size_formatted: "Cloud API".to_string(),
            path: None,
            is_running: true,
            context_length: 131072,
            description: "Ultra-fast LPU inference (500+ tokens/sec)".to_string(),
        },
    ];

    let mut all_models = Vec::new();
    all_models.append(&mut local_ggufs);
    all_models.append(&mut ollama_models);
    all_models.append(&mut sglang_models);

    // If no local models discovered yet, add standard presets
    for p in standard_presets {
        if !all_models.iter().any(|m| m.id == p.id || m.name == p.name) {
            all_models.push(p);
        }
    }

    let (active_model, active_provider) =
        if let Some(first_running) = all_models.iter().find(|m| m.is_running) {
            (first_running.name.clone(), first_running.provider.clone())
        } else if let Some(first) = all_models.first() {
            (first.name.clone(), first.provider.clone())
        } else {
            ("qwen2.5-coder:7b".to_string(), "ollama".to_string())
        };

    Ok(ModelListResponse {
        active_model,
        active_provider,
        local_gguf_count: gguf_count,
        ollama_count,
        models: all_models,
    })
}

#[tauri::command]
pub async fn model_run_prompt(req: RunPromptRequest) -> Result<RunPromptResponse, String> {
    let start = Instant::now();
    info!(
        model = %req.model,
        provider = %req.provider,
        "Agent executing prompt with selected model"
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let default_sys = "You are Oxide-Tech Local Agent — an expert embedded systems, Rust, reverse engineering, and AI agent. Provide accurate, production-grade, zero-filler technical solutions.";
    let sys_prompt = req.system_prompt.as_deref().unwrap_or(default_sys);

    let full_user_prompt = if let Some(stair) = req.stair_context {
        if !stair.is_empty() {
            format!("{}\n\nUser Request:\n{}", stair, req.prompt)
        } else {
            req.prompt.clone()
        }
    } else {
        req.prompt.clone()
    };

    let temp = req.temperature.unwrap_or(0.2);

    let provider = req.provider.to_lowercase();
    let model_name = req.model.trim();

    if provider == "ollama" || model_name.starts_with("ollama:") || provider == "local_gguf" {
        let clean_model = model_name.strip_prefix("ollama:").unwrap_or(model_name);
        let clean_model = clean_model.strip_prefix("local:").unwrap_or(clean_model);
        let base_url = req
            .base_url
            .as_deref()
            .unwrap_or("http://127.0.0.1:11434")
            .trim_end_matches('/');

        let url = format!("{}/api/chat", base_url);
        let body = serde_json::json!({
            "model": clean_model,
            "messages": [
                { "role": "system", "content": sys_prompt },
                { "role": "user", "content": full_user_prompt }
            ],
            "stream": false,
            "options": {
                "temperature": temp,
                "num_predict": req.max_tokens.unwrap_or(4096)
            }
        });

        match client.post(&url).json(&body).send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let status = resp.status();
                    let err_text = resp.text().await.unwrap_or_default();
                    return Ok(RunPromptResponse {
                        text: String::new(),
                        model: clean_model.to_string(),
                        provider: "ollama".to_string(),
                        tokens_used: None,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: Some(format!("Ollama API returned HTTP {}: {}", status, err_text)),
                    });
                }

                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let text = json
                        .get("message")
                        .and_then(|m| m.get("content"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string();

                    let eval_count = json
                        .get("eval_count")
                        .and_then(|c| c.as_u64())
                        .map(|c| c as usize);

                    Ok(RunPromptResponse {
                        text,
                        model: clean_model.to_string(),
                        provider: "ollama".to_string(),
                        tokens_used: eval_count,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: None,
                    })
                } else {
                    Ok(RunPromptResponse {
                        text: String::new(),
                        model: clean_model.to_string(),
                        provider: "ollama".to_string(),
                        tokens_used: None,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: Some("Failed to parse Ollama JSON response".to_string()),
                    })
                }
            }
            Err(e) => Ok(RunPromptResponse {
                text: String::new(),
                model: clean_model.to_string(),
                provider: "ollama".to_string(),
                tokens_used: None,
                latency_ms: start.elapsed().as_millis() as u64,
                error: Some(format!(
                    "Cannot connect to Ollama at {}: {}. Ensure 'ollama serve' or model is active.",
                    base_url, e
                )),
            }),
        }
    } else if provider == "sglang" || provider == "vllm" {
        let base_url = req
            .base_url
            .as_deref()
            .unwrap_or("http://127.0.0.1:30000")
            .trim_end_matches('/');
        let url = format!("{}/v1/chat/completions", base_url);

        let body = serde_json::json!({
            "model": model_name,
            "messages": [
                { "role": "system", "content": sys_prompt },
                { "role": "user", "content": full_user_prompt }
            ],
            "temperature": temp,
            "max_tokens": req.max_tokens.unwrap_or(4096)
        });

        match client.post(&url).json(&body).send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let status = resp.status();
                    let err_text = resp.text().await.unwrap_or_default();
                    return Ok(RunPromptResponse {
                        text: String::new(),
                        model: model_name.to_string(),
                        provider: provider.clone(),
                        tokens_used: None,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: Some(format!("SGLang/vLLM HTTP {}: {}", status, err_text)),
                    });
                }

                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let text = json
                        .get("choices")
                        .and_then(|c| c.as_array())
                        .and_then(|a| a.first())
                        .and_then(|choice| choice.get("message"))
                        .and_then(|m| m.get("content"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string();

                    let tokens = json
                        .get("usage")
                        .and_then(|u| u.get("total_tokens"))
                        .and_then(|t| t.as_u64())
                        .map(|t| t as usize);

                    Ok(RunPromptResponse {
                        text,
                        model: model_name.to_string(),
                        provider,
                        tokens_used: tokens,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: None,
                    })
                } else {
                    Ok(RunPromptResponse {
                        text: String::new(),
                        model: model_name.to_string(),
                        provider,
                        tokens_used: None,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: Some("Failed to parse SGLang completions response".to_string()),
                    })
                }
            }
            Err(e) => Ok(RunPromptResponse {
                text: String::new(),
                model: model_name.to_string(),
                provider,
                tokens_used: None,
                latency_ms: start.elapsed().as_millis() as u64,
                error: Some(format!(
                    "Cannot connect to SGLang endpoint at {}: {}",
                    base_url, e
                )),
            }),
        }
    } else {
        // Cloud / Generic OpenAI API fallback
        let (api_key, api_url, clean_model) = if model_name.contains("gemini") {
            (
                std::env::var("GEMINI_API_KEY")
                    .or_else(|_| std::env::var("GOOGLE_API_KEY"))
                    .ok(),
                "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions"
                    .to_string(),
                "gemini-2.5-flash",
            )
        } else if model_name.contains("groq") {
            (
                std::env::var("GROQ_API_KEY").ok(),
                "https://api.groq.com/openai/v1/chat/completions".to_string(),
                "llama-3.3-70b-versatile",
            )
        } else if model_name.contains("deepseek") {
            (
                std::env::var("DEEPSEEK_API_KEY").ok(),
                "https://api.deepseek.com/v1/chat/completions".to_string(),
                "deepseek-chat",
            )
        } else {
            (
                std::env::var("OPENAI_API_KEY").ok(),
                "https://api.openai.com/v1/chat/completions".to_string(),
                model_name,
            )
        };

        if api_key.is_none() {
            return Ok(RunPromptResponse {
                text: String::new(),
                model: clean_model.to_string(),
                provider: "cloud".to_string(),
                tokens_used: None,
                latency_ms: start.elapsed().as_millis() as u64,
                error: Some(format!(
                    "API key not found for cloud model '{}'. Please select a local GGUF/Ollama model or configure API keys in Settings.",
                    clean_model
                )),
            });
        }

        let body = serde_json::json!({
            "model": clean_model,
            "messages": [
                { "role": "system", "content": sys_prompt },
                { "role": "user", "content": full_user_prompt }
            ],
            "temperature": temp,
            "max_tokens": req.max_tokens.unwrap_or(4096)
        });

        let mut http_req = client.post(&api_url).json(&body);
        if let Some(key) = api_key {
            http_req = http_req.bearer_auth(key);
        }

        match http_req.send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let status = resp.status();
                    let err_text = resp.text().await.unwrap_or_default();
                    return Ok(RunPromptResponse {
                        text: String::new(),
                        model: clean_model.to_string(),
                        provider: "cloud".to_string(),
                        tokens_used: None,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: Some(format!("Cloud API HTTP {}: {}", status, err_text)),
                    });
                }

                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let text = json
                        .get("choices")
                        .and_then(|c| c.as_array())
                        .and_then(|a| a.first())
                        .and_then(|choice| choice.get("message"))
                        .and_then(|m| m.get("content"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("")
                        .to_string();

                    let tokens = json
                        .get("usage")
                        .and_then(|u| u.get("total_tokens"))
                        .and_then(|t| t.as_u64())
                        .map(|t| t as usize);

                    Ok(RunPromptResponse {
                        text,
                        model: clean_model.to_string(),
                        provider: "cloud".to_string(),
                        tokens_used: tokens,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: None,
                    })
                } else {
                    Ok(RunPromptResponse {
                        text: String::new(),
                        model: clean_model.to_string(),
                        provider: "cloud".to_string(),
                        tokens_used: None,
                        latency_ms: start.elapsed().as_millis() as u64,
                        error: Some("Failed to parse Cloud API response".to_string()),
                    })
                }
            }
            Err(e) => Ok(RunPromptResponse {
                text: String::new(),
                model: clean_model.to_string(),
                provider: "cloud".to_string(),
                tokens_used: None,
                latency_ms: start.elapsed().as_millis() as u64,
                error: Some(format!("Cloud network request failed: {}", e)),
            }),
        }
    }
}
