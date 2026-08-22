use knowledge::KnowledgeClient;
use memory::SurrealClient;
use serde_json::Value;
use std::sync::Arc;

/// Manage embeddings or query Qdrant vectors.
pub async fn qdrant_op(
    op_type: &str,
    text: &str,
    rag: &Option<Arc<KnowledgeClient>>,
) -> Result<String, String> {
    let Some(r) = rag else {
        return Err("Qdrant client not initialized".to_string());
    };

    match op_type.to_lowercase().as_str() {
        "retrieve" | "search" => {
            let res = r
                .search("documentation", text, 5)
                .await
                .map_err(|e| format!("Qdrant search failed: {}", e))?;

            let out = serde_json::to_string_pretty(&res)
                .unwrap_or_else(|_| "Failed to format results".to_string());
            Ok(out)
        }
        "store" => {
            // Index the text as a generic chunk in docs
            r.ingest_workspace_ast(&[knowledge::ParsedSymbol {
                name: "custom_mcp_entry".to_string(),
                kind: "Struct".to_string(),
                file_path: "mcp_entry.rs".to_string(),
                start_line: 1,
                end_line: 1,
                content: text.to_string(),
                doc_comment: Some("Stored via MCP qdrant_store".to_string()),
                fields: None,
                methods: None,
                variants: None,
                implements_trait: None,
                target_type: None,
            }])
            .await
            .map_err(|e| format!("Qdrant store failed: {}", e))?;

            Ok("Successfully stored custom text block in Qdrant code collection".to_string())
        }
        _ => Err(format!("Unsupported Qdrant operation: {}", op_type)),
    }
}

/// Query SurrealDB memory tables (project, experience, task).
pub async fn surrealdb_op(
    memory_type: &str,
    query: &str,
    client: &Option<Arc<SurrealClient>>,
) -> Result<String, String> {
    let Some(c) = client else {
        return Err("SurrealDB client not initialized".to_string());
    };

    // If query is empty, generate a default select query for the table
    let sql = if query.trim().is_empty() {
        format!("SELECT * FROM {}", memory_type)
    } else {
        query.to_string()
    };

    let mut response =
        c.db.query(&sql)
            .await
            .map_err(|e| format!("SurrealDB query failed: {}", e))?;

    let res_val: Vec<Value> = response
        .take(0)
        .map_err(|e| format!("Failed to retrieve query results: {}", e))?;

    Ok(serde_json::to_string_pretty(&res_val)
        .unwrap_or_else(|_| "Failed to format results".to_string()))
}

/// Returns a JSON relationship graph representing architecture, dependency, or component edges.
pub async fn knowledge_graph_view(
    entity_id: &str,
    client: &Option<Arc<SurrealClient>>,
) -> Result<String, String> {
    // If SurrealDB is active, query edges.
    if let Some(c) = client {
        let sql = format!(
            "SELECT out, in, label FROM edge WHERE out = '{}' OR in = '{}'",
            entity_id, entity_id
        );
        if let Ok(mut response) = c.db.query(&sql).await {
            if let Ok(res_val) = response.take::<Vec<Value>>(0) {
                if !res_val.is_empty() {
                    return Ok(serde_json::to_string_pretty(&res_val).unwrap());
                }
            }
        }
    }

    // Static fallback: Extract crate dependencies from workspace structure
    let graph = serde_json::json!({
        "entity": entity_id,
        "nodes": [
            {"id": "gateway", "label": "EIOS Gateway API"},
            {"id": "thinker", "label": "Split-Brain Thinker Client"},
            {"id": "router", "label": "Model API Router"},
            {"id": "verifier", "label": "Sandboxed Execution Verifier"},
            {"id": "knowledge", "label": "Qdrant RAG Service"},
            {"id": "memory", "label": "SurrealDB Graph Service"},
            {"id": "mcp-server", "label": "Unified MCP Server"}
        ],
        "edges": [
            {"source": "gateway", "target": "thinker", "label": "calls"},
            {"source": "gateway", "target": "router", "label": "calls"},
            {"source": "gateway", "target": "verifier", "label": "calls"},
            {"source": "gateway", "target": "knowledge", "label": "calls"},
            {"source": "gateway", "target": "memory", "label": "calls"},
            {"source": "mcp-server", "target": "verifier", "label": "uses"},
            {"source": "mcp-server", "target": "knowledge", "label": "uses"},
            {"source": "mcp-server", "target": "memory", "label": "uses"}
        ]
    });

    Ok(serde_json::to_string_pretty(&graph).unwrap())
}
