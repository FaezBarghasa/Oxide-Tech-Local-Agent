use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

// ── Error type ───────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Cannot read config file at '{path}': {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("TOML parse error in '{path}': {source}")]
    Parse {
        path: String,
        #[source]
        source: toml::de::Error,
    },
}

// ── Top-level config ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub gateway: GatewayConfig,
    pub thinker: ModelConfig,
    pub coder: CoderConfig,
    pub rag: RagConfig,
    pub mcp: McpConfig,
    pub auth: AuthConfig,
    #[serde(default)]
    pub knowledge: KnowledgeConfig,
    #[serde(default)]
    pub blog: BlogConfig,
}

// ── Gateway / router ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    pub latency_threshold_ms: u64,
    pub quality_threshold: f32,
    pub max_retries: usize,
    pub host: String,
    pub port: u16,
    pub udp_port: u16,
}

// ── Model / LLM endpoint ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct ModelConfig {
    pub provider: String,
    pub model: String,
    pub base_url: String,
}

// ── Coder section ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct CoderConfig {
    pub online: OnlineCoderConfig,
    pub local: ModelConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OnlineCoderConfig {
    pub primary: ModelConfig,
    pub secondary: ModelConfig,
}

// ── RAG pipeline ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct RagConfig {
    pub collection: String,
    pub top_k: usize,
    pub auto_update_on_startup: bool,
    pub watchlist: Vec<String>,
}

// ── MCP server ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct McpConfig {
    pub transport: String,
    pub tcp_port: u16,
}

// ── Auth config ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
}

// ── Knowledge config ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct KnowledgeConfig {
    /// Hours between automatic knowledge update cycles.
    pub update_interval_hours: u64,
    /// Maximum documentation pages to crawl per crate.
    pub max_pages_per_crate: usize,
    /// Delay in milliseconds between HTTP requests during crawling (polite crawling).
    pub request_delay_ms: u64,
    /// docs.rs crate names to ingest documentation from.
    pub watchlist: Vec<String>,
    /// GitHub org/repo URLs to scrape release notes and README from.
    pub github_repos: Vec<String>,
    /// Named RSS/Atom feed URLs for news ingestion.
    pub news_sources: HashMap<String, String>,
    /// Arbitrary URLs (docs, wikis, homepages) to scrape and index.
    pub custom_urls: Vec<String>,
}

impl Default for KnowledgeConfig {
    fn default() -> Self {
        Self {
            update_interval_hours: 24,
            max_pages_per_crate: 50,
            request_delay_ms: 1000,
            watchlist: vec![
                // ── Core Async & Runtime ──────────────────────────────────
                "tokio".into(),
                "tokio-macros".into(),
                "tokio-util".into(),
                "futures".into(),
                "futures-core".into(),
                "futures-util".into(),
                "futures-channel".into(),
                "futures-io".into(),
                "futures-sink".into(),
                "async-trait".into(),
                "async-std".into(),
                "smol".into(),
                "pin-project".into(),
                "pin-utils".into(),
                // ── Embedded & Bare-Metal (STM32 / ARM / ESP / RP) ────────
                "embassy-executor".into(),
                "embassy-stm32".into(),
                "embassy-time".into(),
                "embassy-sync".into(),
                "embassy-nrf".into(),
                "embassy-rp".into(),
                "rtic".into(),
                "cortex-m".into(),
                "cortex-m-rt".into(),
                "stm32f1xx-hal".into(),
                "stm32f4xx-hal".into(),
                "stm32-hal2".into(),
                "nrf-hal-common".into(),
                "rp2040-hal".into(),
                "embedded-hal".into(),
                "embedded-hal-async".into(),
                "embedded-io".into(),
                "embedded-io-async".into(),
                "embedded-storage".into(),
                "embedded-dma".into(),
                "defmt".into(),
                "defmt-rtt".into(),
                "panic-probe".into(),
                "probe-rs".into(),
                "heapless".into(),
                "nb".into(),
                "bxcan".into(),
                "usb-device".into(),
                "stm32-usbd".into(),
                "esp-hal".into(),
                "esp-idf-hal".into(),
                "esp-idf-svc".into(),
                "linux-embedded-hal".into(),
                "rppal".into(),
                "embedded-graphics".into(),
                "embedded-graphics-simulator".into(),
                // ── Hardware Drivers ──────────────────────────────────────
                "gc9a01-rs".into(),
                "smt160".into(),
                // ── Web Frameworks & Networking ───────────────────────────
                "actix-web".into(),
                "actix-http".into(),
                "actix-rt".into(),
                "axum".into(),
                "tonic".into(),
                "tower".into(),
                "hyper".into(),
                "warp".into(),
                "reqwest".into(),
                "quinn".into(),
                "h3".into(),
                "rumqttc".into(),
                "rumqttd".into(),
                "prost".into(),
                "tungstenite".into(),
                "mio-serial".into(),
                "serialport".into(),
                // ── TLS, Crypto & Security ────────────────────────────────
                "rustls".into(),
                "boring".into(),
                "boring-rustls-provider".into(),
                "aws-lc-rs".into(),
                "ring".into(),
                "rcgen".into(),
                "webpki-roots".into(),
                "rustls-pki-types".into(),
                "sha2".into(),
                // ── WebAssembly (Wasm) & FFI ──────────────────────────────
                "wasm-bindgen".into(),
                "web-sys".into(),
                "js-sys".into(),
                "wasm-bindgen-futures".into(),
                "wasmer".into(),
                "wasmparser".into(),
                "wasm-encoder".into(),
                "wit-bindgen".into(),
                "wit-component".into(),
                "uniffi".into(),
                "cbindgen".into(),
                // ── Serialization & Data Parsing ──────────────────────────
                "serde".into(),
                "serde_json".into(),
                "serde_urlencoded".into(),
                "postcard".into(),
                "ciborium".into(),
                "toml".into(),
                "nom".into(),
                "regex".into(),
                "url".into(),
                "mime".into(),
                "encoding_rs".into(),
                // ── Systems, CLI & OS Interop ─────────────────────────────
                "clap".into(),
                "nix".into(),
                "libc".into(),
                "winapi".into(),
                "windows-sys".into(),
                "redox_syscall".into(),
                "log".into(),
                "tracing".into(),
                "tracing-subscriber".into(),
                "tempfile".into(),
                "fs_extra".into(),
                // ── Database ──────────────────────────────────────────────
                "surrealdb".into(),
                "diesel".into(),
                "sqlx".into(),
                "sea-orm".into(),
                // ── Mathematics, DSP & Scripting ──────────────────────────
                "ndarray".into(),
                "nalgebra".into(),
                "glam".into(),
                "num-traits".into(),
                "num-bigint".into(),
                "fixed".into(),
                "typenum".into(),
                "rustfft".into(),
                "rhai".into(),
                // ── UI & Frontend ─────────────────────────────────────────
                "slint".into(),
                "tauri".into(),
                "dioxus".into(),
                "manganis".into(),
                "iced".into(),
                "egui".into(),
                "leptos".into(),
                "yew".into(),
                // ── Audio, Media & Compression ────────────────────────────
                "symphonia".into(),
                "rodio".into(),
                "cpal".into(),
                "ffmpeg-next".into(),
                "midir".into(),
                "alsa".into(),
                "coremidi".into(),
                "zstd".into(),
                "brotli".into(),
                "flate2".into(),
                "lz4".into(),
                "miniz_oxide".into(),
                // ── Android / JNI ─────────────────────────────────────────
                "jni".into(),
                "ndk".into(),
                "cargo-ndk".into(),
                // ── LLM / AI ──────────────────────────────────────────────
                "candle-core".into(),
                "llama-cpp-rs".into(),
                "ort".into(),
            ],
            github_repos: vec![
                "https://github.com/cloudflare/quiche".into(),
                "https://github.com/stm32-rs".into(),
                "https://github.com/esp-rs".into(),
                "https://github.com/rp-rs".into(),
                "https://github.com/rtic-rs".into(),
                "https://github.com/redox-os".into(),
                "https://github.com/embedded-graphics/embedded-graphics".into(),
                "https://github.com/seanmonstar/reqwest".into(),
                "https://github.com/FaezBarghasa".into(),
                "https://github.com/FaezBarghasa/mqtt-async-embedded".into(),
                "https://github.com/FaezBarghasa/omid".into(),
                "https://github.com/FaezBarghasa/mm-dlp".into(),
            ],
            news_sources: HashMap::from([
                (
                    "this_week_in_rust".into(),
                    "https://this-week-in-rust.org/atom.xml".into(),
                ),
                (
                    "rust_blog".into(),
                    "https://blog.rust-lang.org/feed.xml".into(),
                ),
                (
                    "embedded_rust_blog".into(),
                    "https://blog.rust-embedded.org/feed.xml".into(),
                ),
                (
                    "inside_rust".into(),
                    "https://blog.rust-lang.org/inside-rust/feed.xml".into(),
                ),
            ]),
            custom_urls: vec![
                "https://docs.rs/rppal/latest/rppal/".into(),
                "https://www.redox-os.org/".into(),
                "https://crates.io/crates/ffmpeg-next".into(),
                "https://crates.io/crates/embedded-graphics-simulator".into(),
            ],
        }
    }
}

// ── Blog config ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
pub struct BlogConfig {
    /// Site name in Farsi.
    pub site_name: String,
    /// Site name in English.
    pub site_name_en: String,
    /// Site description in Farsi.
    pub description: String,
    /// Number of posts per page.
    pub posts_per_page: usize,
    /// Whether to auto-publish translated posts (true) or keep as draft (false).
    pub auto_publish: bool,
    /// URL base path for the blog (e.g. "/blog").
    pub base_path: String,
}

impl Default for BlogConfig {
    fn default() -> Self {
        Self {
            site_name: "\u{0627}\u{0646}\u{062c}\u{0645}\u{0646} \u{0631}\u{0627}\u{0633}\u{062a} \u{0627}\u{06cc}\u{0631}\u{0627}\u{0646}".into(), // انجمن راست ایران
            site_name_en: "Rust Iran Community".into(),
            description: "\u{0627}\u{062e}\u{0628}\u{0627}\u{0631} \u{0648} \u{0622}\u{0645}\u{0648}\u{0632}\u{0634}\u{200c}\u{0647}\u{0627}\u{06cc} \u{0631}\u{0627}\u{0633}\u{062a} \u{0628}\u{0647} \u{0641}\u{0627}\u{0631}\u{0633}\u{06cc}".into(), // اخبار و آموزش‌های راست به فارسی
            posts_per_page: 10,
            auto_publish: true,
            base_path: "/blog".into(),
        }
    }
}

// ── Loader ────────────────────────────────────────────────────────────────────

impl AppConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path_str = path.as_ref().to_string_lossy().into_owned();
        let raw = std::fs::read_to_string(path.as_ref()).map_err(|e| ConfigError::Io {
            path: path_str.clone(),
            source: e,
        })?;
        toml::from_str(&raw).map_err(|e| ConfigError::Parse {
            path: path_str,
            source: e,
        })
    }

    pub fn load_default() -> Result<Self, ConfigError> {
        let candidates = ["config.toml", "../config.toml", "../../config.toml"];

        for candidate in &candidates {
            let p = Path::new(candidate);
            if p.exists() {
                return Self::from_file(p);
            }
        }

        Ok(Self::default_config())
    }

    fn default_config() -> Self {
        AppConfig {
            gateway: GatewayConfig {
                latency_threshold_ms: 2000,
                quality_threshold: 0.85,
                max_retries: 5,
                host: "127.0.0.1".to_string(),
                port: 8080,
                udp_port: 8080,
            },
            thinker: ModelConfig {
                provider: "ollama".to_string(),
                model: "qwen2.5-coder:32b-instruct-q4_K_M".to_string(),
                base_url: "http://localhost:11434".to_string(),
            },
            coder: CoderConfig {
                online: OnlineCoderConfig {
                    primary: ModelConfig {
                        provider: "groq".to_string(),
                        model: "llama-3.3-70b-versatile".to_string(),
                        base_url: "https://api.groq.com/openai/v1".to_string(),
                    },
                    secondary: ModelConfig {
                        provider: "mistral".to_string(),
                        model: "codestral-latest".to_string(),
                        base_url: "https://api.mistral.ai/v1".to_string(),
                    },
                },
                local: ModelConfig {
                    provider: "ollama".to_string(),
                    model: "qwen2.5-coder:7b-instruct-q4_K_M".to_string(),
                    base_url: "http://localhost:11434".to_string(),
                },
            },
            rag: RagConfig {
                collection: "rust_rag".to_string(),
                top_k: 8,
                auto_update_on_startup: true,
                watchlist: vec![
                    "embedded-hal".to_string(),
                    "embassy-executor".to_string(),
                    "rtic".to_string(),
                    "cortex-m".to_string(),
                    "defmt".to_string(),
                    "heapless".to_string(),
                ],
            },
            mcp: McpConfig {
                transport: "stdio".to_string(),
                tcp_port: 9090,
            },
            auth: AuthConfig {
                jwt_secret: "eios_secret_key_change_me_in_production".to_string(),
                jwt_expiration_hours: 24,
            },
            knowledge: KnowledgeConfig::default(),
            blog: BlogConfig::default(),
        }
    }
}
