use std::sync::Arc;
use surrealdb_service::client::SurrealClient;
use serde_json::Value;

pub struct GraphRagEngine {
    pub client: Arc<SurrealClient>,
}

impl GraphRagEngine {
    pub fn new(client: Arc<SurrealClient>) -> Self {
        Self { client }
    }

    /// Query the SurrealDB graph for symbol dependencies or code relationships
    pub async fn query_code_relationships(&self, symbol_name: &str) -> Result<Vec<Value>, anyhow::Error> {
        // Query matching symbol and follow relationship edges (e.g. contains, calls, depends_on)
        let mut response = self.client.db
            .query("SELECT ->calls->SymbolRecord as called_symbols FROM SymbolRecord WHERE name = $name")
            .bind(("name", symbol_name))
            .await?;

        let results: Vec<Value> = response.take(0)?;
        Ok(results)
    }

    /// Retrieve component context for a PCB project
    pub async fn query_pcb_board_layout(&self, project_id: &str) -> Result<Vec<Value>, anyhow::Error> {
        let mut response = self.client.db
            .query("SELECT ->contains->component as components FROM project WHERE id = $project_id")
            .bind(("project_id", project_id))
            .await?;

        let results: Vec<Value> = response.take(0)?;
        Ok(results)
    }
}
