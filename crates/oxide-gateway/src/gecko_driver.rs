use anyhow::{Context, Result, anyhow};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::process::Command;

pub struct GeckoSession {
    pub port: u16,
    pub profile_path: PathBuf,
    pub process: Option<tokio::process::Child>,
}

impl GeckoSession {
    pub fn new(port: u16, profile_path: PathBuf) -> Self {
        Self {
            port,
            profile_path,
            process: None,
        }
    }

    /// Spawns Firefox in headless Marionette mode using dedicated user profiles
    pub async fn launch(&mut self) -> Result<()> {
        let child = Command::new("firefox")
            .arg("--headless")
            .arg("--marionette")
            .arg(format!("--marionette-port={}", self.port))
            .arg("--profile")
            .arg(&self.profile_path)
            .arg("--no-remote")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn headless Firefox Marionette process")?;

        self.process = Some(child);

        // Await socket availability on localhost
        for _ in 0..20 {
            tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
            if TcpStream::connect(("127.0.0.1", self.port)).await.is_ok() {
                return Ok(());
            }
        }

        Err(anyhow!(
            "Firefox Marionette socket timed out on port {}",
            self.port
        ))
    }

    /// Executes raw Marionette commands over the native wire protocol
    pub async fn execute_marionette_cmd(&self, cmd: &str, params: Value) -> Result<Value> {
        let mut stream = TcpStream::connect(("127.0.0.1", self.port)).await?;

        let msg = json!([0, 1, cmd, params]).to_string();
        let payload = format!("{}:{}", msg.len(), msg);

        stream.write_all(payload.as_bytes()).await?;

        let mut buf = vec![0u8; 8192];
        let n = stream.read(&mut buf).await?;
        let resp_str = String::from_utf8_lossy(&buf[..n]);

        Ok(json!({ "response": resp_str }))
    }

    /// Dispatches prompt via native hardware event queue
    pub async fn send_prompt(&self, selector: &str, prompt: &str) -> Result<()> {
        let script = format!(
            r#"
            let el = document.querySelector('{}');
            if (el) {{
                el.focus();
                el.value = {};
                el.dispatchEvent(new Event('input', {{ bubbles: true }}));
                let form = el.closest('form');
                if (form) form.dispatchEvent(new Event('submit', {{ bubbles: true }}));
            }}
            "#,
            selector,
            json!(prompt)
        );

        self.execute_marionette_cmd(
            "WebDriver:ExecuteScript",
            json!({
                "script": script,
                "args": []
            }),
        )
        .await?;

        Ok(())
    }

    /// Reads latest completion stream delta from DOM container
    pub async fn poll_response_text(&self, response_selector: &str) -> Result<String> {
        let script = format!(
            r#"
            let el = document.querySelector('{}');
            return el ? el.innerText : '';
            "#,
            response_selector
        );

        let res = self
            .execute_marionette_cmd(
                "WebDriver:ExecuteScript",
                json!({
                    "script": script,
                    "args": []
                }),
            )
            .await?;

        Ok(res["response"].as_str().unwrap_or("").to_string())
    }
}
