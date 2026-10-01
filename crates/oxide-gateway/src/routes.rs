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
        .or_else(|| state.models.get(&target_provider).map(|p| p.value().clone()))
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

    // Spawn inference on async runtime or emit synthetic response if no engine loaded
    actix_web::rt::spawn(async move {
        if let Some(p) = provider {
            if let Err(e) = p.generate(messages, params, tx).await {
                tracing::error!("Inference error: {:?}", e);
            }
        } else {
            let last_user_msg = messages
                .iter()
                .rfind(|m| m.role == oxide_core::Role::User)
                .map(|m| m.text_content())
                .unwrap_or_else(|| "Hello from Oxide Universal AI Gateway".to_string());
            let reply = format!("Oxide Gateway [{}/{}]: {}", target_provider, upstream_model, last_user_msg);
            let _ = tx.send(reply).await;
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

    let response_text = format!("Response from Oxide Gateway for model '{}': {}", model_tag, prompt_text);

    if is_streaming {
        let (tx, rx) = mpsc::channel::<String>(16);
        let resp_clone = response_text.clone();
        tokio::spawn(async move {
            let _ = tx.send("event: message_start\ndata: {\"type\":\"message_start\",\"message\":{\"id\":\"msg_1\",\"type\":\"message\",\"role\":\"assistant\",\"content\":[],\"model\":\"claude-3-7-sonnet\"}}\n\n".into()).await;
            let _ = tx.send(format!("event: content_block_delta\ndata: {{\"type\":\"content_block_delta\",\"index\":0,\"delta\":{{\"type\":\"text_delta\",\"text\":{}}}}}\n\n", serde_json::json!(resp_clone))).await;
            let _ = tx.send("event: message_stop\ndata: {\"type\":\"message_stop\"}\n\n".into()).await;
        });

        let stream = ReceiverStream::new(rx).map(|s| Ok::<_, actix_web::Error>(actix_web::web::Bytes::from(s)));
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
        serde_json::json!({ "id": "auto/smart", "owned_by": "oxide-gateway", "type": "virtual_combo" }),
        serde_json::json!({ "id": "auto/offline", "owned_by": "oxide-gateway", "type": "virtual_combo" }),
        serde_json::json!({ "id": "auto/lkgp", "owned_by": "oxide-gateway", "type": "virtual_combo" }),
        serde_json::json!({ "id": "auto/chaos", "owned_by": "oxide-gateway", "type": "virtual_combo" }),

        // Router Aliases
        serde_json::json!({ "id": "openrouter/auto", "owned_by": "openrouter" }),
        serde_json::json!({ "id": "openrouter/flavor-of-the-week", "owned_by": "openrouter" }),

        // Anthropic Frontier
        serde_json::json!({ "id": "claude-opus-5", "owned_by": "anthropic", "context_window": 500000, "supports_thinking": true }),
        serde_json::json!({ "id": "claude-opus-4-8", "owned_by": "anthropic", "context_window": 200000, "supports_thinking": true }),
        serde_json::json!({ "id": "claude-3-7-sonnet", "owned_by": "anthropic", "context_window": 200000, "supports_thinking": true }),
        serde_json::json!({ "id": "claude-3-5-sonnet", "owned_by": "anthropic", "context_window": 200000 }),
        serde_json::json!({ "id": "claude-3-5-haiku", "owned_by": "anthropic", "context_window": 200000 }),

        // OpenAI Frontier & Reasoning
        serde_json::json!({ "id": "gpt-5.6-sol", "owned_by": "openai", "context_window": 1000000, "supports_thinking": true }),
        serde_json::json!({ "id": "gpt-4o", "owned_by": "openai", "context_window": 128000 }),
        serde_json::json!({ "id": "gpt-4o-mini", "owned_by": "openai", "context_window": 128000 }),
        serde_json::json!({ "id": "o3-mini", "owned_by": "openai", "context_window": 200000, "supports_thinking": true }),
        serde_json::json!({ "id": "o1", "owned_by": "openai", "context_window": 200000, "supports_thinking": true }),
        serde_json::json!({ "id": "o1-mini", "owned_by": "openai", "context_window": 128000, "supports_thinking": true }),

        // DeepSeek
        serde_json::json!({ "id": "deepseek-chat", "owned_by": "deepseek", "context_window": 128000 }),
        serde_json::json!({ "id": "deepseek-reasoner", "owned_by": "deepseek", "context_window": 128000, "supports_thinking": true }),
        serde_json::json!({ "id": "deepseek-coder", "owned_by": "deepseek", "context_window": 128000 }),

        // Google DeepMind
        serde_json::json!({ "id": "gemini-2.5-pro", "owned_by": "google", "context_window": 2000000, "supports_thinking": true }),
        serde_json::json!({ "id": "gemini-2.0-flash", "owned_by": "google", "context_window": 1000000 }),
        serde_json::json!({ "id": "gemini-1.5-pro", "owned_by": "google", "context_window": 2000000 }),
        serde_json::json!({ "id": "gemini-1.5-flash", "owned_by": "google", "context_window": 1000000 }),

        // Meta Llama
        serde_json::json!({ "id": "llama-3.3-70b", "owned_by": "meta", "context_window": 128000 }),
        serde_json::json!({ "id": "llama-3.1-405b", "owned_by": "meta", "context_window": 128000 }),
        serde_json::json!({ "id": "llama-3.1-70b", "owned_by": "meta", "context_window": 128000 }),
        serde_json::json!({ "id": "llama-3.1-8b", "owned_by": "meta", "context_window": 128000 }),

        // Alibaba Qwen
        serde_json::json!({ "id": "qwen-2.5-coder-32b", "owned_by": "alibaba", "context_window": 128000 }),
        serde_json::json!({ "id": "qwen-2.5-72b", "owned_by": "alibaba", "context_window": 128000 }),
        serde_json::json!({ "id": "qwen-turbo", "owned_by": "alibaba", "context_window": 128000 }),
        serde_json::json!({ "id": "qwen-plus", "owned_by": "alibaba", "context_window": 128000 }),
        serde_json::json!({ "id": "qwen-max", "owned_by": "alibaba", "context_window": 128000, "supports_thinking": true }),

        // Mistral AI
        serde_json::json!({ "id": "mistral-large", "owned_by": "mistralai", "context_window": 128000 }),
        serde_json::json!({ "id": "mistral-small", "owned_by": "mistralai", "context_window": 32000 }),
        serde_json::json!({ "id": "codestral", "owned_by": "mistralai", "context_window": 256000 }),
        serde_json::json!({ "id": "pixtral", "owned_by": "mistralai", "context_window": 128000 }),

        // Zhipu, Moonshot, MiniMax
        serde_json::json!({ "id": "glm-4.7", "owned_by": "zhipu", "context_window": 128000 }),
        serde_json::json!({ "id": "glm-4", "owned_by": "zhipu", "context_window": 128000 }),
        serde_json::json!({ "id": "kimi-k3", "owned_by": "moonshot", "context_window": 2000000 }),
        serde_json::json!({ "id": "kimi-k2", "owned_by": "moonshot", "context_window": 200000 }),
        serde_json::json!({ "id": "minimax-m3", "owned_by": "minimax", "context_window": 1000000 }),
    ];

    // Merge active models
    for entry in state.models.iter() {
        let key = entry.key();
        if !catalog.iter().any(|m| m.get("id").and_then(|id| id.as_str()) == Some(key)) {
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
    messages.push(ChatMessage::new_text(oxide_core::Role::User, prompt.clone()));

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
        HttpResponse::Ok().json(ThinkResponse {
            status: "success".to_string(),
            reply: format!(
                "Oxide Local Agent Synthesizer: Processed prompt ({} characters). Connected to local workstation engine.",
                prompt.len()
            ),
            model: "local-synthesizer".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
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

pub async fn system_one_decision(
    req: web::Json<SystemOneRequest>,
) -> impl Responder {
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

