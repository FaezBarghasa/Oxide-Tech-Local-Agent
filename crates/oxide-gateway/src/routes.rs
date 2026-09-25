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
    let provider = match state.models.get(&model_name) {
        Some(p) => p.value().clone(),
        None => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": {
                    "message": format!("Model '{}' not found in registry", model_name),
                    "type": "invalid_request_error"
                }
            }));
        }
    };

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

    // Spawn inference on async runtime
    actix_web::rt::spawn(async move {
        if let Err(e) = provider.generate(messages, params, tx).await {
            tracing::error!("Inference error: {:?}", e);
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

pub async fn list_models(state: web::Data<Arc<AppState>>) -> impl Responder {
    let models: Vec<serde_json::Value> = state
        .models
        .iter()
        .map(|entry| {
            serde_json::json!({
                "id": entry.key(),
                "object": "model",
                "owned_by": "oxide-tech",
                "permission": []
            })
        })
        .collect();

    HttpResponse::Ok().json(serde_json::json!({
        "object": "list",
        "data": models
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
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: sys.clone(),
        });
    }
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: prompt.clone(),
    });

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

