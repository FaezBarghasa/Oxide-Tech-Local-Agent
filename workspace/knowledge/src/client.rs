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
use qdrant_client::qdrant::{PointStruct, SearchPoints, UpsertPointsBuilder};

use crate::ast::ParsedSymbol;
use crate::collections::setup_qdrant_collections;
use common::error::{EiosError, Result};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct EiosChunk {
    pub text: String,
    pub source: String,
    pub crate_name: Option<String>,
    pub version: Option<String>,
    pub file_path: Option<String>,
    pub symbol_name: Option<String>,
}

pub struct KnowledgeClient {
    pub qdrant: Qdrant,
    pub embedder: Mutex<TextEmbedding>,
}

impl KnowledgeClient {
    pub async fn new() -> Result<Self> {
        let qdrant_url =
            std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".to_string());

        let qdrant = Qdrant::from_url(&qdrant_url)
            .build()
            .map_err(|e| EiosError::VectorStore(e.to_string()))?;

        // Initialize BGE Small EN v1.5 embedder locally
        let embedder = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(true),
        )
        .map_err(|e| EiosError::Internal(format!("Failed to initialize FastEmbed: {}", e)))?;

        let client = Self {
            qdrant,
            embedder: Mutex::new(embedder),
        };

        setup_qdrant_collections(&client.qdrant)
            .await
            .map_err(|e| EiosError::VectorStore(e.to_string()))?;

        Ok(client)
    }

    /// Crawl and ingest docs.rs pages for a specific crate and version into "documentation" collection.
    pub async fn ingest_crate_docs(&self, crate_name: &str, version: &str) -> Result<()> {
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
            .build()
            .map_err(|e| EiosError::Internal(e.to_string()))?;

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
                embedder
                    .embed(chunks.clone(), None)
                    .map_err(|e| EiosError::Internal(format!("Embedding failed: {}", e)))?
            };

            let mut points = Vec::new();
            for (i, (chunk, embedding)) in
                chunks.into_iter().zip(embeddings.into_iter()).enumerate()
            {
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
                .try_into()
                .map_err(|e| EiosError::VectorStore(format!("Payload conversion error: {}", e)))?;

                points.push(PointStruct::new(point_uuid.to_string(), embedding, payload));
            }

            if !points.is_empty() {
                self.qdrant
                    .upsert_points(UpsertPointsBuilder::new("documentation", points))
                    .await
                    .map_err(|e| EiosError::VectorStore(e.to_string()))?;
            }
        }

        info!(
            "Successfully ingested docs for crate {} v{}",
            crate_name, version
        );
        Ok(())
    }

    /// Ingest research results from External Research & Perception Layer with strict provenance
    pub async fn ingest_research_result(
        &self,
        url: &str,
        text: &str,
        engine: &str,
        confidence_score: f32,
        title: Option<&str>,
    ) -> Result<()> {
        let chunks = chunk_text(text, 400, 50);
        if chunks.is_empty() {
            return Ok(());
        }

        let embeddings = {
            let mut embedder = self.embedder.lock().unwrap();
            embedder
                .embed(chunks.clone(), None)
                .map_err(|e| EiosError::Internal(format!("Embedding failed: {}", e)))?
        };

        let mut points = Vec::new();
        for (i, (chunk, embedding)) in chunks.into_iter().zip(embeddings.into_iter()).enumerate() {
            let point_uuid =
                Uuid::new_v5(&Uuid::NAMESPACE_URL, format!("research://{}#{}", url, i).as_bytes());

            let payload: Payload = serde_json::json!({
                "text": chunk,
                "source": "external_research",
                "source_url": url,
                "engine": engine,
                "confidence_score": confidence_score,
                "title": title.unwrap_or(""),
                "timestamp": Utc::now().to_rfc3339(),
            })
            .try_into()
            .map_err(|e| EiosError::VectorStore(format!("Payload conversion error: {}", e)))?;

            points.push(PointStruct::new(point_uuid.to_string(), embedding, payload));
        }

        let point_count = points.len();
        if !points.is_empty() {
            self.qdrant
                .upsert_points(UpsertPointsBuilder::new("documentation", points))
                .await
                .map_err(|e| EiosError::VectorStore(e.to_string()))?;
        }

        info!(
            "Grounded {} research chunks for {} via engine: {} (confidence: {:.2})",
            point_count,
            url,
            engine,
            confidence_score
        );
        Ok(())
    }

    /// Ingest parsed workspace AST symbols into the "code" collection.
    pub async fn ingest_workspace_ast(&self, symbols: &[ParsedSymbol]) -> Result<()> {
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
            embedder
                .embed(chunks.clone(), None)
                .map_err(|e| EiosError::Internal(format!("Embedding failed: {}", e)))?
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

            let point_uuid = Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                format!("workspace://{}#{}#{}", file_path, symbol_name, i).as_bytes(),
            );

            let mut payload_json = payload_val;
            payload_json
                .as_object_mut()
                .unwrap()
                .insert("text".to_string(), serde_json::Value::String(chunk));

            let payload: Payload = payload_json
                .try_into()
                .map_err(|e| EiosError::VectorStore(format!("Payload conversion error: {}", e)))?;

            points.push(PointStruct::new(point_uuid.to_string(), embedding, payload));
        }

        if !points.is_empty() {
            self.qdrant
                .upsert_points(UpsertPointsBuilder::new("code", points))
                .await
                .map_err(|e| EiosError::VectorStore(e.to_string()))?;
        }

        info!("Workspace AST ingestion complete.");
        Ok(())
    }

    /// Generic semantic search over any collection.
    pub async fn search(
        &self,
        collection: &str,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<EiosChunk>> {
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        let embeddings = {
            let mut embedder = self.embedder.lock().unwrap();
            embedder
                .embed(vec![query.to_string()], None)
                .map_err(|e| EiosError::Internal(format!("Embedding failed: {}", e)))?
        };
        if embeddings.is_empty() {
            return Ok(Vec::new());
        }

        let query_vector = embeddings[0].clone();

        let response = self
            .qdrant
            .search_points(SearchPoints {
                collection_name: collection.to_string(),
                vector: query_vector,
                limit: top_k as u64,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await
            .map_err(|e| EiosError::VectorStore(e.to_string()))?;

        let mut results = Vec::new();
        for point in response.result {
            let val =
                serde_json::to_value(&point.payload).map_err(|e| EiosError::Serialization(e))?;

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

            results.push(EiosChunk {
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

    /// Ingest named RSS/Atom feed into "news_raw" collection.
    pub async fn ingest_rss_feed(&self, feed_url: &str, source_name: &str) -> Result<()> {
        info!(
            "Starting RSS feed ingestion for {} from {}",
            source_name, feed_url
        );
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| EiosError::Internal(e.to_string()))?;

        let resp = client
            .get(feed_url)
            .send()
            .await
            .map_err(|e| EiosError::Internal(format!("Failed to fetch RSS: {}", e)))?;

        if !resp.status().is_success() {
            return Err(EiosError::Internal(format!(
                "HTTP error {} fetching RSS: {}",
                resp.status(),
                feed_url
            )));
        }

        let body = resp
            .text()
            .await
            .map_err(|e| EiosError::Internal(format!("Failed to read RSS body: {}", e)))?;

        let entries = crate::rss::parse_feed(&body);
        info!(
            "Parsed {} entries from RSS feed: {}",
            entries.len(),
            source_name
        );

        for entry in entries {
            let doc_text = format!(
                "Title: {}\n\nPublished: {}\n\nSummary/Content: {}",
                entry.title, entry.published, entry.summary
            );
            let chunks = chunk_text(&doc_text, 400, 50);
            if chunks.is_empty() {
                continue;
            }

            let embeddings = {
                let mut embedder = self.embedder.lock().unwrap();
                embedder
                    .embed(chunks.clone(), None)
                    .map_err(|e| EiosError::Internal(format!("Embedding failed: {}", e)))?
            };

            let mut points = Vec::new();
            for (i, (chunk, embedding)) in
                chunks.into_iter().zip(embeddings.into_iter()).enumerate()
            {
                let point_uuid = Uuid::new_v5(
                    &Uuid::NAMESPACE_URL,
                    format!("rss:{}#{}#{}", feed_url, entry.link, i).as_bytes(),
                );

                let payload: Payload = serde_json::json!({
                    "text": chunk,
                    "source": source_name,
                    "title": entry.title,
                    "url": entry.link,
                    "published": entry.published,
                    "timestamp": Utc::now().to_rfc3339(),
                })
                .try_into()
                .map_err(|e| EiosError::VectorStore(format!("Payload conversion error: {}", e)))?;

                points.push(PointStruct::new(point_uuid.to_string(), embedding, payload));
            }

            if !points.is_empty() {
                self.qdrant
                    .upsert_points(UpsertPointsBuilder::new("news_raw", points))
                    .await
                    .map_err(|e| EiosError::VectorStore(e.to_string()))?;
            }
        }

        Ok(())
    }

    /// Ingest a GitHub repository or organization into the "documentation" collection.
    pub async fn ingest_github_releases(&self, repo_url: &str) -> Result<()> {
        info!("Ingesting GitHub source: {}", repo_url);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("EIOS-Agent/1.0")
            .build()
            .map_err(|e| EiosError::Internal(e.to_string()))?;

        let clean_url = repo_url.trim_end_matches('/');
        let parts: Vec<&str> = clean_url.split("github.com/").collect();
        if parts.len() < 2 {
            warn!("Invalid GitHub URL format: {}", repo_url);
            return Ok(());
        }

        let path = parts[1];
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if path_segments.len() >= 2 {
            // Repository
            let owner = path_segments[0];
            let repo = path_segments[1];

            let releases_feed = format!("https://github.com/{}/{}/releases.atom", owner, repo);
            if let Ok(resp) = client.get(&releases_feed).send().await {
                if resp.status().is_success() {
                    if let Ok(body) = resp.text().await {
                        let entries = crate::rss::parse_feed(&body);
                        info!("Parsed {} releases for {}/{}", entries.len(), owner, repo);
                        for entry in entries {
                            let doc_text = format!(
                                "GitHub Release: {}/{}\nTitle: {}\nPublished: {}\nContent: {}",
                                owner, repo, entry.title, entry.published, entry.summary
                            );
                            let chunks = chunk_text(&doc_text, 400, 50);
                            self.embed_and_upsert_chunks(
                                "documentation",
                                chunks,
                                &entry.link,
                                "github_release",
                                Some(repo),
                            )
                            .await?;
                        }
                    }
                }
            }

            if let Ok(resp) = client.get(clean_url).send().await {
                if resp.status().is_success() {
                    if let Ok(html) = resp.text().await {
                        let text = extract_text_from_html(&html);
                        let chunks = chunk_text(&text, 400, 50);
                        self.embed_and_upsert_chunks(
                            "documentation",
                            chunks,
                            clean_url,
                            "github_readme",
                            Some(repo),
                        )
                        .await?;
                    }
                }
            }
        } else if path_segments.len() == 1 {
            // Organization/User
            if let Ok(resp) = client.get(clean_url).send().await {
                if resp.status().is_success() {
                    if let Ok(html) = resp.text().await {
                        let text = extract_text_from_html(&html);
                        let chunks = chunk_text(&text, 400, 50);
                        self.embed_and_upsert_chunks(
                            "documentation",
                            chunks,
                            clean_url,
                            "github_org",
                            None,
                        )
                        .await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Scrape and ingest arbitrary web page into the "documentation" collection.
    pub async fn ingest_custom_url(&self, url: &str) -> Result<()> {
        info!("Scraping custom URL: {}", url);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("EIOS-Agent/1.0")
            .build()
            .map_err(|e| EiosError::Internal(e.to_string()))?;

        let resp = client
            .get(url)
            .send()
            .await
            .map_err(|e| EiosError::Internal(format!("Failed to fetch URL {}: {}", url, e)))?;

        if !resp.status().is_success() {
            warn!("HTTP error {} for URL: {}", resp.status(), url);
            return Ok(());
        }

        let html = resp
            .text()
            .await
            .map_err(|e| EiosError::Internal(format!("Failed to read body from {}: {}", url, e)))?;

        let text = extract_text_from_html(&html);
        let chunks = chunk_text(&text, 400, 50);

        let crate_name = if url.contains("rppal") {
            Some("rppal")
        } else if url.contains("ffmpeg-next") {
            Some("ffmpeg-next")
        } else if url.contains("embedded-graphics-simulator") {
            Some("embedded-graphics-simulator")
        } else {
            None
        };

        self.embed_and_upsert_chunks("documentation", chunks, url, "custom_url", crate_name)
            .await?;
        Ok(())
    }

    async fn embed_and_upsert_chunks(
        &self,
        collection: &str,
        chunks: Vec<String>,
        url: &str,
        source: &str,
        crate_name: Option<&str>,
    ) -> Result<()> {
        if chunks.is_empty() {
            return Ok(());
        }

        let embeddings = {
            let mut embedder = self.embedder.lock().unwrap();
            embedder
                .embed(chunks.clone(), None)
                .map_err(|e| EiosError::Internal(format!("Embedding failed: {}", e)))?
        };

        let mut points = Vec::new();
        for (i, (chunk, embedding)) in chunks.into_iter().zip(embeddings.into_iter()).enumerate() {
            let point_uuid = Uuid::new_v5(
                &Uuid::NAMESPACE_URL,
                format!("{}:{}#{}", source, url, i).as_bytes(),
            );

            let payload: Payload = serde_json::json!({
                "text": chunk,
                "source": source,
                "crate_name": crate_name,
                "url": url,
                "timestamp": Utc::now().to_rfc3339(),
            })
            .try_into()
            .map_err(|e| EiosError::VectorStore(format!("Payload conversion error: {}", e)))?;

            points.push(PointStruct::new(point_uuid.to_string(), embedding, payload));
        }

        if !points.is_empty() {
            self.qdrant
                .upsert_points(UpsertPointsBuilder::new(collection, points))
                .await
                .map_err(|e| EiosError::VectorStore(e.to_string()))?;
        }

        Ok(())
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
    let mut chunks = Vec::new();

    if words.is_empty() {
        return chunks;
    }

    let mut i = 0;
    while i < words.len() {
        let end = (i + chunk_size).min(words.len());
        let chunk = words[i..end].join(" ");
        chunks.push(chunk);

        if end == words.len() {
            break;
        }

        i += chunk_size - overlap;
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
