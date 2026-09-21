use actix_web::{web, HttpResponse, Responder};
use futures_util::StreamExt;
use oxide_core::{ChatMessage, GenerationParams};
use oxide_state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Deserialize)]
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

pub async fn chat_completions(
    state: web::Data<Arc<AppState>>,
    req: web::Json<ChatCompletionRequest>,
) -> impl Responder {
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

    let params = GenerationParams {
        temperature: req.temperature.unwrap_or(0.7),
        top_p: req.top_p.unwrap_or(0.95),
        max_tokens: req.max_tokens,
        stop: None,
        stream: req.stream.unwrap_or(true),
    };

    let (tx, rx) = mpsc::channel::<String>(128);
    let messages = req.messages.clone();

    // Spawn inference on async runtime
    tokio::spawn(async move {
        if let Err(e) = provider.generate(messages, params, tx).await {
            tracing::error!("Inference error: {:?}", e);
        }
    });

    let chunk_id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let created = chrono::Utc::now().timestamp();
    let model_tag = model_name.clone();

    let stream = ReceiverStream::new(rx).map(move |token| {
        let chunk = ChatCompletionChunk {
            id: chunk_id.clone(),
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
        .content_type("text/event-stream")
        .streaming(stream)
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
