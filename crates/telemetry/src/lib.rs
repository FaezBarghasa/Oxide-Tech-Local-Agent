//! # Telemetry
//!
//! OpenTelemetry tracing and metrics initializer for the Oxide-Tech agent system.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use telemetry::TelemetryConfig;
//!
//! #[tokio::main]
//! async fn main() {
//!     let _guard = TelemetryConfig::from_env().init().await.unwrap();
//!     // application code…
//! }
//! ```

use anyhow::{Context, Result};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, util::SubscriberInitExt};

/// Configuration for the telemetry subsystem.
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// OTLP gRPC endpoint (e.g. `http://localhost:4317`)
    pub otlp_endpoint: Option<String>,
    /// Service name reported to the collector
    pub service_name: String,
    /// Log filter directive (e.g. `info,router=debug`)
    pub log_filter: String,
    /// Emit structured JSON logs instead of human-readable text
    pub json_logs: bool,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            otlp_endpoint: None,
            service_name: "oxide-tech-agent".to_string(),
            log_filter: "info".to_string(),
            json_logs: false,
        }
    }
}

impl TelemetryConfig {
    /// Build configuration from environment variables:
    /// - `OTLP_ENDPOINT` — OTLP gRPC collector URL
    /// - `OTEL_SERVICE_NAME` — service name label
    /// - `RUST_LOG` — log filter (defaults to `info`)
    /// - `LOG_JSON=true` — emit JSON-formatted logs
    pub fn from_env() -> Self {
        Self {
            otlp_endpoint: std::env::var("OTLP_ENDPOINT").ok(),
            service_name: std::env::var("OTEL_SERVICE_NAME")
                .unwrap_or_else(|_| "oxide-tech-agent".to_string()),
            log_filter: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            json_logs: std::env::var("LOG_JSON").as_deref() == Ok("true"),
        }
    }

    /// Initialise the global tracing subscriber (with optional OTLP export).
    ///
    /// Returns a `TelemetryGuard` — dropping it flushes all pending spans and logs.
    pub async fn init(self) -> Result<TelemetryGuard> {
        let env_filter = EnvFilter::try_new(&self.log_filter)
            .context("invalid RUST_LOG filter")?;

        if let Some(ref endpoint) = self.otlp_endpoint {
            // ── OTLP exporter (tracing) ─────────────────────────────────────
            let exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint);

            let tracer_provider = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .with_trace_config(sdktrace::Config::default().with_resource(
                    opentelemetry_sdk::Resource::new(vec![opentelemetry::KeyValue::new(
                        "service.name",
                        self.service_name.clone(),
                    )]),
                ))
                .install_batch(runtime::Tokio)
                .context("failed to install OTLP tracer")?;

            let tracer = tracer_provider.tracer(self.service_name.clone());
            let otel_layer = OpenTelemetryLayer::new(tracer);

            if self.json_logs {
                Registry::default()
                    .with(env_filter)
                    .with(tracing_subscriber::fmt::layer().json())
                    .with(otel_layer)
                    .init();
            } else {
                Registry::default()
                    .with(env_filter)
                    .with(tracing_subscriber::fmt::layer())
                    .with(otel_layer)
                    .init();
            }

            Ok(TelemetryGuard {
                provider: Some(tracer_provider),
            })
        } else {
            // ── Local stdout/stderr only ─────────────────────────────────────
            if self.json_logs {
                Registry::default()
                    .with(env_filter)
                    .with(tracing_subscriber::fmt::layer().json())
                    .init();
            } else {
                Registry::default()
                    .with(env_filter)
                    .with(tracing_subscriber::fmt::layer())
                    .init();
            }

            Ok(TelemetryGuard { provider: None })
        }
    }
}

/// RAII guard — when dropped, flushes all pending OTLP spans to the collector.
pub struct TelemetryGuard {
    provider: Option<opentelemetry_sdk::trace::TracerProvider>,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.provider.take() {
            if let Err(e) = provider.shutdown() {
                eprintln!("telemetry: failed to flush OTLP spans on shutdown: {e}");
            }
        }
    }
}
