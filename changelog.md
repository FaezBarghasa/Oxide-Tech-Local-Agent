# Oxide-Tech Local Agent OS: Changelog

All notable changes to the Oxide-Tech Local Agent OS backend codebase are documented here.

---

## [v0.2.0-beta] - 2026-06-21

This release implements a series of upgrades focused on database performance, multi-protocol gateways, asynchronous worker safety, robust code-generation verifications, and a fully automated daily knowledge scraper & curation translation pipeline.

### Upgrades & New Features

#### 0. Daily Knowledge Scraper & RTL Farsi Curation Blog (Rust Iran Community)
- **Watchlist Ingestion**: Expanded the knowledge crawler to scrape documentation for 140+ Rust crates across 15 domains, 12 GitHub release feeds, and custom URLs politely (using request delays).
- **RSS/Atom Feed Parser**: Added a lightweight, robust RSS/Atom parser built on top of the `select` HTML parsing library to digest news updates.
- **LLM-Powered Curation**: Integrated a translation pipeline that feeds raw articles to the Thinker LLM to curate, summarize, and translate top stories into RTL Farsi.
- **SurrealDB Curation Service**: Implemented BlogPost model storage and tag/pagination CRUD operations, with an automated background scheduler tokio task.
- **RTL Farsi Website Templates**: Designed a premium glassmorphic, RTL-oriented dark mode theme for the blog index, tag listings, posts, and Atom feed endpoint `/blog/feed.xml`.

#### 1. SurrealDB v3 Migration
- **Dependency Version bump**: Upgraded SurrealDB core package to version `3` and integrated `surrealdb-types = "3.1.5"`.
- **API Enhancements**:
  - Replaced deprecated string-based `Thing` representations with the formal `RecordId` structure.
  - Derived `SurrealValue` on database models (`Project`, `Component`, `CompilationRecord`) to guarantee type-safety at query level.
  - Adapted symbol database methods in `crates/surrealdb-service/src/client.rs` to compile against SurrealDB v3 engine interfaces (e.g. `connect` accepting mem/ws schemas, transactional queries, and `take` operations).

#### 2. Parallel HTTP/3 (QUIC) Gateway Integration
- **H3 and Quinn Setup**: Built a parallel UDP endpoint on port `8080` alongside the standard Actix-web TCP listener.
- **Alt-Svc Negotiation**: Added the standard `Alt-Svc: h3=":8080"; ma=86400` header to all Actix responses to allow HTTP/2-enabled clients to automatically migrate to HTTP/3.
- **TLS 1.3 Requirements**: Integrated `rustls` (configured with `ring` crypto provider) to handle the mandatory TLS 1.3 handshake required for HTTP/3 over QUIC.

#### 3. Vector Database Engine Optimization (Qdrant v1.18.0)
- **Qdrant Client Setup**: Modified `crates/qdrant-service/src/client.rs` to match the builder-style API of newer `qdrant-client` crates.
- **Collection Setup**: Leveraged `CreateCollectionBuilder` and `VectorParamsBuilder` to create the `datasheets` search collection dynamically on startup.
- **Hybrid Search**: Prepared the schema to handle sparse BM25 token weights combined with dense cosine-distance vectors for improved search accuracy.

#### 4. Async Worker Thread Protection
- **Decoupled Blocking Operations**: To prevent long-running filesystem reads or walks from stalling Actix worker threads, directory traversals and parsing loops were rewritten using `tokio::task::spawn_blocking`.
- **WebSocket Executor Isolation**: Solved stream safety panics. Actix Web's `MessageStream` is not thread-safe (`!Send`); websocket message handlers were migrated to execute exclusively on Actix's thread-local executor using `actix_web::rt::spawn` instead of `tokio::spawn`.

#### 5. Additional Endpoints
- **Liveness & Readiness Probes**: Implemented `/health/live` and `/health/ready` check modules in `crates/api/src/routes/health.rs`.
- **Prometheus Metrics**: Configured metric gathering endpoints at `/metrics`.
- **Repository Structure Explorer**: Added the `/api/repository/structure` endpoint to generate recursive directory context trees dynamically for AI models.

---

## [v0.1.0] - 2026-06-21

### Initial Release
- **Core Platform**: Integrated Actix-web HTTP gateway running cargo execution sandboxes.
- **Python Bridge**: Implemented bridges to control KiCad, Skidl, and PyTorch fine-tuning models.
- **AI Agent framework**: Initial draft of the Chief Planner-Execution auto-healing logic.
