use crate::client::SurrealClient;
use crate::schema::CompilationRecord;
use surrealdb::Error;
use surrealdb_types::RecordId;

pub async fn log_compilation(
    client: &SurrealClient,
    project_id: RecordId,
    success: bool,
    stdout: &str,
    stderr: &str,
) -> Result<CompilationRecord, Error> {
    let record = CompilationRecord {
        id: None,
        project_id,
        success,
        stdout: stdout.to_string(),
        stderr: stderr.to_string(),
        created_at: chrono::Utc::now(),
    };

    let created: Option<CompilationRecord> =
        client.db.create("compilation").content(record).await?;

    created.ok_or_else(|| Error::thrown("Failed to log compilation".to_string()))
}

pub async fn get_compilation_history(
    client: &SurrealClient,
    project_id: RecordId,
) -> Result<Vec<CompilationRecord>, Error> {
    let mut response = client
        .db
        .query("SELECT * FROM compilation WHERE project_id = $project_id ORDER BY created_at DESC")
        .bind(("project_id", project_id))
        .await?;
    let records: Vec<CompilationRecord> = response.take(0)?;
    Ok(records)
}
