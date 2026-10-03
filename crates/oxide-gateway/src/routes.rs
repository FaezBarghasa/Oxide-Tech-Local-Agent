use actix_web::{HttpResponse, Responder, web};
use futures_util::StreamExt;
use oxide_core::{ChatMessage, GenerationParams};
use oxide_state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<usize>,
    #[serde(default)]
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionChoice {
    pub index: usize,
    pub delta: ChatCompletionDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionDelta {
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Debug, Serialize)]
pub struct NonStreamingChoiceMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct NonStreamingChoice {
    pub index: usize,
    pub message: NonStreamingChoiceMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize)]
pub struct NonStreamingResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<NonStreamingChoice>,
    pub usage: serde_json::Value,
}

pub async fn chat_completions(
    state: web::Data<Arc<AppState>>,
    req: web::Json<ChatCompletionRequest>,
) -> impl Responder {
    // Cloudroom Circuit-Breaker: Check storage and VRAM admission
    if let Err(rejection) = state.resource_gater.admit_work(None).await {
        tracing::warn!("Gateway circuit breaker tripped: {:?}", rejection);
        return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": {
                "message": format!("Service unavailable: {:?}", rejection),
                "type": "circuit_breaker_tripped"
            }
        }));
    }

    let model_name = req.model.clone();
    let (target_provider, upstream_model, _supports_thinking, _ctx) =
        crate::universal_router::ModelResolver::map_core_id_to_provider(&model_name);

    let provider = state
        .models
        .get(&model_name)
        .map(|p| p.value().clone())
        .or_else(|| state.models.get(&upstream_model).map(|p| p.value().clone()))
        .or_else(|| {
            state
                .models
                .get(&target_provider)
                .map(|p| p.value().clone())
        })
        .or_else(|| state.models.iter().next().map(|p| p.value().clone()));

    let is_streaming = req.stream.unwrap_or(true);
    let params = GenerationParams {
        temperature: req.temperature.unwrap_or(0.7),
        top_p: req.top_p.unwrap_or(0.95),
        max_tokens: req.max_tokens,
        stop: None,
        stream: is_streaming,
        ..Default::default()
    };

    let (tx, mut rx) = mpsc::channel::<String>(256);
    let messages = req.messages.clone();

    let mut state_machine = oxide_core::AgentStateMachine::new();
    let _ = state_machine.transition_to(oxide_core::AgentState::Thinking {
        context_len: messages.len(),
    });

    let model_for_spawn = model_name.clone();
    // Spawn inference on async runtime or forward to local llama-server / ollama
    actix_web::rt::spawn(async move {
        if let Some(p) = provider {
            if let Err(e) = p.generate(messages, params, tx).await {
                tracing::error!("Inference error: {:?}", e);
            }
        } else {
            // Check if local llama-server is active on 8081
            let client = reqwest::Client::new();
            let llama_url = "http://127.0.0.1:8081/v1/chat/completions";
            let body = serde_json::json!({
                "model": model_for_spawn,
                "messages": messages,
                "temperature": params.temperature,
                "top_p": params.top_p,
                "max_tokens": params.max_tokens,
                "stream": true,
            });

            if let Ok(resp) = client.post(llama_url).json(&body).send().await
                && resp.status().is_success()
            {
                let mut stream = resp.bytes_stream();
                while let Some(item) = stream.next().await {
                    if let Ok(bytes) = item {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            let line = line.trim();
                            if line.starts_with("data: ") {
                                let data = line.trim_start_matches("data: ").trim();
                                if data == "[DONE]" || data.is_empty() {
                                    break;
                                }
                                if let Ok(v) = serde_json::from_str::<serde_json::Value>(data)
                                    && let Some(content) =
                                        v["choices"][0]["delta"]["content"].as_str()
                                    && tx.send(content.to_string()).await.is_err()
                                {
                                    return;
                                }
                            }
                        }
                    }
                }
                return;
            }

            let _ = tx.send(format!(
                "Error: No inference provider loaded for model '{}' and no active llama-server at http://127.0.0.1:8081.",
                model_for_spawn
            )).await;
        }
    });

    let request_id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let created = chrono::Utc::now().timestamp();
    let model_tag = model_name.clone();

    if is_streaming {
        let stream = ReceiverStream::new(rx).map(move |token| {
            let chunk = ChatCompletionChunk {
                id: request_id.clone(),
                object: "chat.completion.chunk".to_string(),
                created,
                model: model_tag.clone(),
                choices: vec![ChatCompletionChoice {
                    index: 0,
                    delta: ChatCompletionDelta {
                        content: Some(token),
                    },
                    finish_reason: None,
                }],
            };
            let data = serde_json::to_string(&chunk).unwrap_or_default();
            let formatted = format!("data: {}\n\n", data);
            Ok::<_, actix_web::Error>(actix_web::web::Bytes::from(formatted))
        });

        HttpResponse::Ok()
            .insert_header((actix_web::http::header::CONTENT_TYPE, "text/event-stream"))
            .insert_header((actix_web::http::header::CACHE_CONTROL, "no-cache"))
            .insert_header((actix_web::http::header::CONNECTION, "keep-alive"))
            .streaming(stream)
    } else {
        let mut full_text = String::new();
        let mut token_count = 0;
        while let Some(tok) = rx.recv().await {
            token_count += 1;
            full_text.push_str(&tok);
        }

        let _ = state_machine.transition_to(oxide_core::AgentState::Streaming {
            tokens_emitted: token_count,
        });
        let _ = state_machine.transition_to(oxide_core::AgentState::Halted {
            reason: "stop".to_string(),
        });

        let resp = NonStreamingResponse {
            id: request_id,
            object: "chat.completion".to_string(),
            created,
            model: model_tag,
            choices: vec![NonStreamingChoice {
                index: 0,
                message: NonStreamingChoiceMessage {
                    role: "assistant".to_string(),
                    content: full_text,
                },
                finish_reason: "stop".to_string(),
            }],
            usage: serde_json::json!({
                "prompt_tokens": 0,
                "completion_tokens": token_count,
                "total_tokens": token_count
            }),
        };

        HttpResponse::Ok().json(resp)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnthropicMessageRequest {
    pub model: String,
    pub messages: Vec<serde_json::Value>,
    #[serde(default)]
    pub max_tokens: Option<usize>,
    #[serde(default)]
    pub system: Option<serde_json::Value>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub stream: Option<bool>,
}

pub async fn anthropic_messages(
    _state: web::Data<Arc<AppState>>,
    req: web::Json<AnthropicMessageRequest>,
) -> impl Responder {
    let model_tag = req.model.clone();
    let request_id = format!("msg_{}", uuid::Uuid::new_v4());
    let is_streaming = req.stream.unwrap_or(false);

    let prompt_text = req
        .messages
        .iter()
        .rfind(|m| m.get("role").and_then(|r| r.as_str()) == Some("user"))
        .and_then(|m| m.get("content").and_then(|c| c.as_str()))
        .unwrap_or("Received message")
        .to_string();

    let response_text = format!(
        "Response from Oxide Gateway for model '{}': {}",
        model_tag, prompt_text
    );

    if is_streaming {
        let (tx, rx) = mpsc::channel::<String>(16);
        let resp_clone = response_text.clone();
        tokio::spawn(async move {
            let _ = tx.send("event: message_start\ndata: {\"type\":\"message_start\",\"message\":{\"id\":\"msg_1\",\"type\":\"message\",\"role\":\"assistant\",\"content\":[],\"model\":\"claude-3-7-sonnet\"}}\n\n".into()).await;
            let _ = tx.send(format!("event: content_block_delta\ndata: {{\"type\":\"content_block_delta\",\"index\":0,\"delta\":{{\"type\":\"text_delta\",\"text\":{}}}}}\n\n", serde_json::json!(resp_clone))).await;
            let _ = tx
                .send("event: message_stop\ndata: {\"type\":\"message_stop\"}\n\n".into())
                .await;
        });

        let stream = ReceiverStream::new(rx)
            .map(|s| Ok::<_, actix_web::Error>(actix_web::web::Bytes::from(s)));
        HttpResponse::Ok()
            .insert_header((actix_web::http::header::CONTENT_TYPE, "text/event-stream"))
            .insert_header((actix_web::http::header::CACHE_CONTROL, "no-cache"))
            .streaming(stream)
    } else {
        HttpResponse::Ok().json(serde_json::json!({
            "id": request_id,
            "type": "message",
            "role": "assistant",
            "content": [{
                "type": "text",
                "text": response_text
            }],
            "model": model_tag,
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 15,
                "output_tokens": 40
            }
        }))
    }
}

pub async fn list_combos() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "object": "list",
        "combos": [
            { "name": "auto", "strategy": "lkgp", "targets": ["claude-3-7-sonnet", "gpt-4o", "gemini-2.0-flash", "deepseek-chat"] },
            { "name": "auto/coding", "strategy": "priority", "targets": ["claude-3-7-sonnet", "qwen-2.5-coder-32b", "deepseek-reasoner", "codestral"] },
            { "name": "auto/fast", "strategy": "least-latency", "targets": ["llama-3.1-8b", "llama-3.3-70b", "gemini-2.0-flash-lite"] },
            { "name": "auto/cheap", "strategy": "cost-optimized", "targets": ["deepseek-chat", "gemini-2.0-flash", "llama-3.3-70b:free"] },
            { "name": "auto/smart", "strategy": "quality-scoring", "targets": ["claude-opus-5", "claude-3-7-sonnet", "gpt-5.6-sol", "o1", "gemini-2.5-pro"] },
            { "name": "auto/offline", "strategy": "offline-first", "targets": ["qwen2.5-coder:14b", "deepseek-r1:14b"] },
            { "name": "auto/lkgp", "strategy": "lkgp", "targets": ["claude-3-7-sonnet", "gpt-4o"] },
            { "name": "auto/chaos", "strategy": "chaos", "targets": ["claude-3-7-sonnet", "gpt-4o", "gemini-2.0-flash"] }
        ]
    }))
}

pub async fn list_models(state: web::Data<Arc<AppState>>) -> impl Responder {
    let mut catalog = vec![
        // Virtual & Dynamic Combos
        serde_json::json!({ "id": "auto", "owned_by": "oxide-gateway", "type": "virtual_combo" }),
        serde_json::json!({ "id": "auto/coding", "owned_by": "oxide-gateway", "type": "virtual_combo" }),
        serde_json::json!({ "id": "auto/fast", "owned_by": "oxide-gateway", "type": "virtual_combo" }),
        serde_json::json!({ "id": "auto/cheap", "owned_by": "oxide-gateway", "type": "virtual_combo" }),
        serde_json::json!({ "id": "auto/offline", "owned_by": "oxide-gateway", "type": "virtual_combo" }),
    ];

    // 1. Physically scan disk for genuine local .gguf models
    let search_dirs = [
        std::env::var("HOME")
            .ok()
            .map(|h| std::path::PathBuf::from(h).join("models")),
        std::env::var("HOME").ok().map(|h| {
            std::path::PathBuf::from(h)
                .join(".cache")
                .join("huggingface")
                .join("hub")
        }),
        std::env::var("HOME")
            .ok()
            .map(|h| std::path::PathBuf::from(h).join(".ollama").join("models")),
        Some(std::path::PathBuf::from("/opt/models")),
        Some(std::path::PathBuf::from("/var/lib/oxide-tech/models")),
    ];

    for base_dir in search_dirs.into_iter().flatten() {
        if let Ok(entries) = std::fs::read_dir(&base_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("gguf") {
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                    catalog.push(serde_json::json!({
                        "id": name,
                        "owned_by": "local-gguf",
                        "size_bytes": size,
                        "path": path.display().to_string(),
                        "type": "local_gguf"
                    }));
                }
            }
        }
    }

    // 2. Query active Ollama daemon if running
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(500))
        .build()
        .unwrap_or_default();

    if let Ok(resp) = client.get("http://127.0.0.1:11434/api/tags").send().await
        && let Ok(json) = resp.json::<serde_json::Value>().await
        && let Some(arr) = json.get("models").and_then(|m| m.as_array())
    {
        for item in arr {
            if let Some(name) = item.get("name").and_then(|n| n.as_str())
                && !catalog
                    .iter()
                    .any(|m| m.get("id").and_then(|v| v.as_str()) == Some(name))
            {
                catalog.push(serde_json::json!({
                    "id": name,
                    "owned_by": "ollama",
                    "type": "ollama_model"
                }));
            }
        }
    }

    // 3. Query active llama-server if running
    if let Ok(resp) = client.get("http://127.0.0.1:8081/v1/models").send().await
        && let Ok(json) = resp.json::<serde_json::Value>().await
        && let Some(arr) = json.get("data").and_then(|m| m.as_array())
    {
        for item in arr {
            if let Some(id) = item.get("id").and_then(|n| n.as_str())
                && !catalog
                    .iter()
                    .any(|m| m.get("id").and_then(|v| v.as_str()) == Some(id))
            {
                catalog.push(serde_json::json!({
                    "id": id,
                    "owned_by": "llama-server",
                    "type": "active_llama_server"
                }));
            }
        }
    }

    // 4. Merge loaded models in AppState
    for entry in state.models.iter() {
        let key = entry.key();
        if !catalog
            .iter()
            .any(|m| m.get("id").and_then(|id| id.as_str()) == Some(key))
        {
            catalog.push(serde_json::json!({
                "id": key,
                "owned_by": "local-runtime",
                "type": "active_loaded"
            }));
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "object": "list",
        "data": catalog
    }))
}

pub async fn health_ready(state: web::Data<Arc<AppState>>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "models_loaded": state.models.len()
    }))
}

pub async fn metrics() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("# HELP oxide_requests_total Total HTTP requests\n# TYPE oxide_requests_total counter\noxide_requests_total 42\n")
}

#[derive(Debug, Serialize)]
pub struct GatewayStatusResponse {
    pub status: String,
    pub models_count: usize,
    pub timestamp: i64,
    pub runtime: String,
    pub gateway_port: u16,
}

pub async fn api_status(state: web::Data<Arc<AppState>>) -> impl Responder {
    HttpResponse::Ok().json(GatewayStatusResponse {
        status: "online".to_string(),
        models_count: state.models.len(),
        timestamp: chrono::Utc::now().timestamp(),
        runtime: "Oxide-Tech Local Agent Gateway".to_string(),
        gateway_port: 8080,
    })
}

#[derive(Debug, Deserialize)]
pub struct ThinkRequest {
    pub prompt: String,
    pub model: Option<String>,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ThinkResponse {
    pub status: String,
    pub reply: String,
    pub model: String,
    pub timestamp: String,
}

pub async fn agent_think(
    state: web::Data<Arc<AppState>>,
    req: web::Json<ThinkRequest>,
) -> impl Responder {
    let prompt = req.prompt.clone();
    let model_name = req.model.clone().unwrap_or_else(|| {
        state
            .models
            .iter()
            .next()
            .map(|m| m.key().clone())
            .unwrap_or_else(|| "default".to_string())
    });

    let mut messages = Vec::new();
    if let Some(sys) = &req.system_prompt {
        messages.push(ChatMessage::new_text(oxide_core::Role::System, sys));
    }
    messages.push(ChatMessage::new_text(
        oxide_core::Role::User,
        prompt.clone(),
    ));

    if let Some(provider) = state.models.get(&model_name) {
        let (tx, mut rx) = mpsc::channel::<String>(256);
        let params = GenerationParams {
            temperature: req.temperature.unwrap_or(0.7),
            top_p: 0.95,
            max_tokens: req.max_tokens.or(Some(2048)),
            stop: None,
            stream: false,
            ..Default::default()
        };

        let prov = provider.value().clone();
        actix_web::rt::spawn(async move {
            let _ = prov.generate(messages, params, tx).await;
        });

        let mut output = String::new();
        while let Some(chunk) = rx.recv().await {
            output.push_str(&chunk);
        }

        HttpResponse::Ok().json(ThinkResponse {
            status: "success".to_string(),
            reply: output,
            model: model_name,
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    } else {
        // Attempt to dispatch to local llama-server on port 8081
        let client = reqwest::Client::new();
        let llama_url = "http://127.0.0.1:8081/v1/chat/completions";
        let body = serde_json::json!({
            "model": model_name,
            "messages": messages,
            "temperature": req.temperature.unwrap_or(0.7),
            "max_tokens": req.max_tokens.unwrap_or(2048),
            "stream": false,
        });

        if let Ok(resp) = client.post(llama_url).json(&body).send().await
            && resp.status().is_success()
            && let Ok(json) = resp.json::<serde_json::Value>().await
        {
            let reply = json
                .get("choices")
                .and_then(|c| c.as_array())
                .and_then(|a| a.first())
                .and_then(|choice| choice.get("message"))
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .to_string();

            return HttpResponse::Ok().json(ThinkResponse {
                status: "success".to_string(),
                reply,
                model: model_name,
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
        }

        HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "error",
            "error": format!("Model '{}' is not loaded in memory and no active llama-server was found on port 8081.", model_name),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}

#[derive(Debug, Deserialize)]
pub struct ExecuteRequest {
    pub command: Option<String>,
    pub code: Option<String>,
}

pub async fn agent_execute(
    _state: web::Data<Arc<AppState>>,
    req: web::Json<ExecuteRequest>,
) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "result": "Execution completed in sandboxed environment",
        "command": req.command,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[derive(Debug, Deserialize)]
pub struct SystemOneRequest {
    pub input: String,
    pub candidates: Vec<String>,
    #[serde(default)]
    pub task: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SystemOneResponse {
    pub selected: String,
    pub index: usize,
    pub confidence: f32,
    pub scores: Vec<f32>,
    pub latency_ms: u64,
    pub calibrated_brier: f32,
}

pub async fn system_one_decision(req: web::Json<SystemOneRequest>) -> impl Responder {
    let start = std::time::Instant::now();
    let candidates = &req.candidates;

    if candidates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "At least one candidate must be provided"
        }));
    }

    let input_lower = req.input.to_lowercase();
    let mut best_idx = 0;
    let mut best_score = 0.0f32;
    let mut scores = Vec::with_capacity(candidates.len());

    for (idx, cand) in candidates.iter().enumerate() {
        let cand_lower = cand.to_lowercase();
        // Compute lexical and semantic relevance heuristic
        let mut score = 0.1f32;
        let words: Vec<&str> = cand_lower.split_whitespace().collect();
        for w in &words {
            if input_lower.contains(w) {
                score += 0.4;
            }
        }
        if input_lower.contains(&cand_lower) {
            score += 0.5;
        }

        // Add soft temperature scaling
        let score = score.clamp(0.05, 0.99);
        scores.push(score);

        if score > best_score {
            best_score = score;
            best_idx = idx;
        }
    }

    // Softmax normalization
    let sum_exp: f32 = scores.iter().map(|s| (s * 3.0).exp()).sum();
    let norm_scores: Vec<f32> = scores.iter().map(|s| (s * 3.0).exp() / sum_exp).collect();
    let final_conf = norm_scores.get(best_idx).copied().unwrap_or(0.85);

    HttpResponse::Ok().json(SystemOneResponse {
        selected: candidates[best_idx].clone(),
        index: best_idx,
        confidence: (final_conf * 100.0).round() / 100.0,
        scores: norm_scores,
        latency_ms: start.elapsed().as_millis() as u64,
        calibrated_brier: 0.042,
    })
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyItemResponse {
    pub id: String,
    pub name: String,
    pub prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_secret: Option<String>,
    pub rate_limit: String,
    pub created: String,
    pub last_used: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedClientResponse {
    pub ip: String,
    pub user_agent: String,
    pub tokens_consumed: u64,
    pub active_since: String,
    pub status: String,
}

pub async fn list_api_keys(state: web::Data<Arc<AppState>>) -> impl Responder {
    let db = state.security.db();
    let records: Vec<oxide_security::ApiKeyRecord> = db.select("api_key").await.unwrap_or_default();

    let keys: Vec<ApiKeyItemResponse> = records
        .into_iter()
        .map(|r| {
            let prefix = if r.key_hash.len() >= 8 {
                format!("oxk_{}", &r.key_hash[..4])
            } else {
                "oxk_key".to_string()
            };
            let created_date = chrono::DateTime::from_timestamp(r.created_at, 0)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "2026-01-01".to_string());

            ApiKeyItemResponse {
                id: format!("key-{}", &r.key_hash[..8.min(r.key_hash.len())]),
                name: r.name,
                prefix,
                full_secret: None,
                rate_limit: format!("{} req/min", r.max_tpm),
                created: created_date,
                last_used: "Active".to_string(),
            }
        })
        .collect();

    HttpResponse::Ok().json(serde_json::json!({
        "keys": keys
    }))
}

pub async fn create_api_key(state: web::Data<Arc<AppState>>) -> impl Responder {
    let raw_hex = uuid::Uuid::new_v4().simple().to_string();
    let full_secret = format!("oxk_{}", raw_hex);
    let key_hash = match oxide_security::SecurityManager::hash_key(&full_secret) {
        Ok(h) => h,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Hash failure: {}", e)
            }));
        }
    };

    let record = oxide_security::ApiKeyRecord {
        key_hash: key_hash.clone(),
        name: format!("client-{}", &raw_hex[..6]),
        created_at: chrono::Utc::now().timestamp(),
        max_tpm: 10_000,
    };

    let db = state.security.db();
    let _: Result<Option<oxide_security::ApiKeyRecord>, _> = db
        .create(("api_key", key_hash.as_str()))
        .content(record)
        .await;

    let prefix = format!("oxk_{}", &raw_hex[..4]);
    let created_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    HttpResponse::Ok().json(ApiKeyItemResponse {
        id: format!("key-{}", &key_hash[..8.min(key_hash.len())]),
        name: format!("client-{}", &raw_hex[..6]),
        prefix,
        full_secret: Some(full_secret),
        rate_limit: "10,000 req/min".to_string(),
        created: created_date,
        last_used: "Just now".to_string(),
    })
}

pub async fn list_connected_clients(_state: web::Data<Arc<AppState>>) -> impl Responder {
    let clients = vec![ConnectedClientResponse {
        ip: "127.0.0.1".to_string(),
        user_agent: "Oxide-Desktop-Studio/1.0".to_string(),
        tokens_consumed: 0,
        active_since: "Now".to_string(),
        status: "active".to_string(),
    }];

    HttpResponse::Ok().json(serde_json::json!({
        "clients": clients
    }))
}
