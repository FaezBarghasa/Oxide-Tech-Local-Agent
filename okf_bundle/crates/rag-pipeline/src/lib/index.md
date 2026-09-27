# lib

## Classs

- [RagChunk](RagChunk.md) — [derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
- [RagPipeline](RagPipeline.md)

## Functions

- [chunk_text](chunk_text.md) — ── Text Chunking Helper ──────────────────────────────────────────────────────
- [extract_text_from_html](extract_text_from_html.md) — ── Text Extraction Helper ────────────────────────────────────────────────────
- [format_symbol](format_symbol.md) — ── Symbol Formatting Helpers ──────────────────────────────────────────────────
- [format_symbol_summary](format_symbol_summary.md)
- [ingest_crate_docs](ingest_crate_docs.md) — Crawl and ingest docs.rs pages for a specific crate and version.
- [ingest_crate_docs](ingest_crate_docs_1.md) — Crawl and ingest docs.rs pages for a specific crate and version.
- [ingest_workspace_ast](ingest_workspace_ast.md) — Ingest parsed workspace AST symbols into the vector database.
- [ingest_workspace_ast](ingest_workspace_ast_1.md) — Ingest parsed workspace AST symbols into the vector database.
- [new](new.md) — Initialize the RAG pipeline by connecting to Qdrant, initializing FastEmbed, and connecting to SurrealDB.
- [new](new_1.md) — Initialize the RAG pipeline by connecting to Qdrant, initializing FastEmbed, and connecting to SurrealDB.
- [search](search.md) — Dense vector search over the collection.
- [search](search_1.md) — Dense vector search over the collection.
- [setup_collection](setup_collection.md) — Set up the vector collection if it doesn't already exist.
- [setup_collection](setup_collection_1.md) — Set up the vector collection if it doesn't already exist.
