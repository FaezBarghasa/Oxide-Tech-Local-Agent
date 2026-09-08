#![allow(clippy::new_without_default, clippy::useless_conversion)]

use chrono::Utc;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::sync::Mutex;
use tracing::{info, warn};
use uuid::Uuid;

use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use qdrant_client::Payload;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, SearchPoints, UpsertPointsBuilder,
    VectorParamsBuilder,
};

use qdrant_service::client::QdrantServiceClient;
use surrealdb_service::client::SurrealClient;
use tree_sitter_service::ast::ParsedSymbol;

pub mod fable_router;
pub mod okf;
pub mod updater;

const COLLECTION_NAME: &str = "rust_rag";

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct RagChunk {
    pub text: String,
    pub source: String,
    pub crate_name: Option<String>,
    pub version: Option<String>,
    pub file_path: Option<String>,
    pub symbol_name: Option<String>,
}

pub struct RagPipeline {
    qdrant: Qdrant,
    embedder: Mutex<TextEmbedding>,
    pub surreal: SurrealClient,
}

impl RagPipeline {
    /// Initialize the RAG pipeline by connecting to Qdrant, initializing FastEmbed, and connecting to SurrealDB.
    pub async fn new() -> Result<Self, anyhow::Error> {
        let qdrant_service = QdrantServiceClient::new()?;
        let qdrant = qdrant_service.client;

        // Initialize BGE Small EN v1.5 embedder locally
        let embedder = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(true),
        )?;

        let surreal = SurrealClient::new().await?;

        let pipeline = Self {
            qdrant,
            embedder: Mutex::new(embedder),
            surreal,
        };

        pipeline.setup_collection().await?;

        Ok(pipeline)
    }

    /// Set up the vector collection if it doesn't already exist.
    pub async fn setup_collection(&self) -> Result<(), anyhow::Error> {
        let collections = self.qdrant.list_collections().await?;
        let exists = collections
            .collections
            .iter()
            .any(|c| c.name == COLLECTION_NAME);
        if !exists {
            info!("Creating Qdrant collection: {}", COLLECTION_NAME);
            self.qdrant
                .create_collection(
                    CreateCollectionBuilder::new(COLLECTION_NAME)
                        .vectors_config(VectorParamsBuilder::new(384, Distance::Cosine)),
                )
                .await?;
        }
        Ok(())
    }

    /// Crawl and ingest docs.rs pages for a specific crate and version.
    pub async fn ingest_crate_docs(
        &self,
        crate_name: &str,
        version: &str,
    ) -> Result<(), anyhow::Error> {
        let module_name = crate_name.replace('-', "_");
        let base_url = format!(
            "https://docs.rs/{}/{}/{}/",
            crate_name, version, module_name
        );
        let index_url = format!("{}index.html", base_url);

        info!(
            "Starting doc ingestion for crate {} v{} at {}",
            crate_name, version, index_url
        );

        let mut pages_to_fetch = vec![index_url.clone()];
        let mut fetched_pages = HashSet::new();
        let mut docs = Vec::new();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        // Crawl up to 30 main doc pages
        while let Some(url) = pages_to_fetch.pop() {
            if fetched_pages.contains(&url) || fetched_pages.len() >= 30 {
                continue;
            }

            info!("Fetching page: {}", url);
            fetched_pages.insert(url.clone());

            let Ok(resp) = client.get(&url).send().await else {
                warn!("Failed to fetch URL: {}", url);
                continue;
            };

            if !resp.status().is_success() {
                warn!("HTTP error {} for URL: {}", resp.status(), url);
                continue;
            }

            let Ok(html) = resp.text().await else {
                continue;
            };

            let text = extract_text_from_html(&html);
            if !text.trim().is_empty() {
                docs.push((url.clone(), text));
            }

            // Find child links if this is an index page
            if url.ends_with("index.html") {
                let doc = Document::from(html.as_str());
                for link in doc.find(Name("a")).filter_map(|n| n.attr("href")) {
                    if link.starts_with("struct.")
                        || link.starts_with("trait.")
                        || link.starts_with("enum.")
                    {
                        let full_link = format!("{}{}", base_url, link);
                        pages_to_fetch.push(full_link);
                    }
                }
            }
        }

        info!(
            "Crawled {} pages for crate {}. Chunking and embedding...",
            docs.len(),
            crate_name
        );

        for (url, text) in docs {
            let chunks = chunk_text(&text, 400, 50);
            if chunks.is_empty() {
                continue;
            }

            let embeddings = {
                let mut embedder = self.embedder.lock().unwrap();
                embedder.embed(chunks.clone(), None)?
            };

            let mut points = Vec::new();
            for (i, (chunk, embedding)) in
                chunks.into_iter().zip(embeddings.into_iter()).enumerate()
            {
                // Generate a deterministic UUID based on the URL and chunk index
                let point_uuid =
                    Uuid::new_v5(&Uuid::NAMESPACE_URL, format!("{}#{}", url, i).as_bytes());

                let payload: Payload = serde_json::json!({
                    "text": chunk,
                    "source": "docs.rs",
                    "crate_name": crate_name,
                    "version": version,
                    "url": url,
                    "timestamp": Utc::now().to_rfc3339(),
                })
                .try_into()?;

                points.push(PointStruct::new(point_uuid.to_string(), embedding, payload));
            }

            if !points.is_empty() {
                self.qdrant
                    .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, points))
                    .await?;
            }
        }

        info!(
            "Successfully ingested docs for crate {} v{}",
            crate_name, version
        );
        Ok(())
    }

    /// Ingest parsed workspace AST symbols into the vector database.
    pub async fn ingest_workspace_ast(
        &self,
        symbols: &[ParsedSymbol],
    ) -> Result<(), anyhow::Error> {
        info!(
            "Ingesting {} workspace AST symbols into RAG...",
            symbols.len()
        );

        let mut chunks = Vec::new();
        let mut payloads = Vec::new();

        for sym in symbols {
            let text = format_symbol(sym);
            chunks.push(text);

            let payload = serde_json::json!({
                "text": format_symbol_summary(sym),
                "source": "workspace",
                "file_path": sym.file_path,
                "symbol_name": sym.name,
                "timestamp": Utc::now().to_rfc3339(),
            });
            payloads.push(payload);
        }

        if chunks.is_empty() {
            return Ok(());
        }

        let embeddings = {
            let mut embedder = self.embedder.lock().unwrap();
            embedder.embed(chunks.clone(), None)?
        };

        let mut points = Vec::new();
        for (i, ((chunk, embedding), payload_val)) in chunks
            .into_iter()
            .zip(embeddings.into_iter())
            .zip(payloads.into_iter())
            .enumerate()
        {
            let file_path = payload_val
                .get("file_path")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let symbol_name = payload_val
                .get("symbol_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            // Deterministic UUID for the symbol
            let point_uuid = Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                format!("workspace://{}#{}#{}", file_path, symbol_name, i).as_bytes(),
            );

            // Reconstruct full payload
            let mut payload_json = payload_val;
            payload_json
                .as_object_mut()
                .unwrap()
                .insert("text".to_string(), serde_json::Value::String(chunk));

            let payload: Payload = payload_json.try_into()?;

            points.push(PointStruct::new(point_uuid.to_string(), embedding, payload));
        }

        if !points.is_empty() {
            self.qdrant
                .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, points))
                .await?;
        }

        info!("Workspace AST ingestion complete.");
        Ok(())
    }

    /// Dense vector search over the collection.
    pub async fn search(&self, query: &str, top_k: usize) -> Result<Vec<RagChunk>, anyhow::Error> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let embeddings = {
            let mut embedder = self.embedder.lock().unwrap();
            embedder.embed(vec![query.to_string()], None)?
        };
        if embeddings.is_empty() {
            return Ok(Vec::new());
        }

        let query_vector = embeddings[0].clone();

        let response = self
            .qdrant
            .search_points(SearchPoints {
                collection_name: COLLECTION_NAME.to_string(),
                vector: query_vector,
                limit: top_k as u64,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await?;

        let mut results = Vec::new();
        for point in response.result {
            let val = serde_json::to_value(&point.payload)?;

            let text = val
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let source = val
                .get("source")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let crate_name = val
                .get("crate_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let version = val
                .get("version")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let file_path = val
                .get("file_path")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let symbol_name = val
                .get("symbol_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            results.push(RagChunk {
                text,
                source,
                crate_name,
                version,
                file_path,
                symbol_name,
            });
        }

        Ok(results)
    }
}

// ── Text Extraction Helper ────────────────────────────────────────────────────

fn extract_text_from_html(html: &str) -> String {
    let doc = Document::from(html);
    if let Some(main) = doc.find(Name("main")).next() {
        main.text()
    } else if let Some(body) = doc.find(Name("body")).next() {
        body.text()
    } else {
        doc.find(Name("html"))
            .next()
            .map(|n| n.text())
            .unwrap_or_default()
    }
}

// ── Text Chunking Helper ──────────────────────────────────────────────────────

fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return Vec::new();
    }

    let step = if chunk_size > overlap {
        chunk_size - overlap
    } else {
        1
    };
    let estimated_chunks = (words.len() + step - 1) / step;
    let mut chunks = Vec::with_capacity(estimated_chunks);

    let mut i = 0;
    while i < words.len() {
        let end = (i + chunk_size).min(words.len());
        // Estimate byte capacity for joining slice
        let byte_len: usize = words[i..end].iter().map(|w| w.len() + 1).sum();
        let mut chunk = String::with_capacity(byte_len);
        for (idx, word) in words[i..end].iter().enumerate() {
            if idx > 0 {
                chunk.push(' ');
            }
            chunk.push_str(word);
        }
        chunks.push(chunk);

        if end == words.len() {
            break;
        }

        i += step;
    }

    chunks
}

// ── Symbol Formatting Helpers ──────────────────────────────────────────────────

fn format_symbol(sym: &ParsedSymbol) -> String {
    let mut s = format!(
        "Symbol: {}\nKind: {}\nFile: {}\nLine Range: {}-{}\n",
        sym.name, sym.kind, sym.file_path, sym.start_line, sym.end_line
    );
    if let Some(ref doc) = sym.doc_comment {
        s.push_str(&format!("Doc Comment:\n{}\n", doc));
    }
    if let Some(ref fields) = sym.fields {
        s.push_str("Fields:\n");
        for f in fields {
            s.push_str(&format!("  - {}: {}\n", f.name, f.r#type));
        }
    }
    if let Some(ref methods) = sym.methods {
        s.push_str("Methods:\n");
        for m in methods {
            s.push_str(&format!("  - {}\n", m.signature));
        }
    }
    s.push_str(&format!("Content:\n```rust\n{}\n```", sym.content));
    s
}

fn format_symbol_summary(sym: &ParsedSymbol) -> String {
    let mut s = format!(
        "Symbol: {}\nKind: {}\nFile: {}\n",
        sym.name, sym.kind, sym.file_path
    );
    if let Some(ref doc) = sym.doc_comment {
        s.push_str(&format!(
            "Doc Comment: {}\n",
            doc.lines().next().unwrap_or("")
        ));
    }
    s
}
