use actix_web::{Error, HttpRequest, HttpResponse, get, web};
use actix_ws::Message;
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct WsBroadcaster {
    pub compilation_tx: broadcast::Sender<String>,
    pub agent_progress_tx: broadcast::Sender<String>,
}

impl WsBroadcaster {
    pub fn new() -> Self {
        let (compilation_tx, _) = broadcast::channel(100);
        let (agent_progress_tx, _) = broadcast::channel(100);
        Self {
            compilation_tx,
            agent_progress_tx,
        }
    }
}

impl Default for WsBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

#[get("/ws/compilation")]
pub async fn handle_ws_compilation(
    req: HttpRequest,
    body: web::Payload,
    broadcaster: web::Data<Arc<WsBroadcaster>>,
) -> Result<HttpResponse, Error> {
    let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;
    let mut rx = broadcaster.compilation_tx.subscribe();

    actix_web::rt::spawn(async move {
        let mut session_clone = session.clone();
        let writer_task = actix_web::rt::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if session_clone.text(msg).await.is_err() {
                    break;
                }
            }
        });

        while let Some(Ok(msg)) = msg_stream.next().await {
            if let Message::Close(reason) = msg {
                let _ = session.close(reason).await;
                break;
            }
        }
        writer_task.abort();
    });

    Ok(response)
}

#[get("/ws/agent-progress")]
pub async fn handle_ws_agent_progress(
    req: HttpRequest,
    body: web::Payload,
    broadcaster: web::Data<Arc<WsBroadcaster>>,
) -> Result<HttpResponse, Error> {
    let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;
    let mut rx = broadcaster.agent_progress_tx.subscribe();

    actix_web::rt::spawn(async move {
        let mut session_clone = session.clone();
        let writer_task = actix_web::rt::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if session_clone.text(msg).await.is_err() {
                    break;
                }
            }
        });

        while let Some(Ok(msg)) = msg_stream.next().await {
            if let Message::Close(reason) = msg {
                let _ = session.close(reason).await;
                break;
            }
        }
        writer_task.abort();
    });

    Ok(response)
}
