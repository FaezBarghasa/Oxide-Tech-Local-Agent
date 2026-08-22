use crate::client::SurrealClient;
use serde::{Deserialize, Serialize};
use surrealdb::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStats {
    pub total_projects: u64,
    pub total_components: u64,
    pub compilation_success_rate: f64,
}

pub async fn get_project_stats(
    client: &SurrealClient,
) -> Result<ProjectStats, Error> {
    let mut response = client.db
        .query("SELECT count() AS count FROM project GROUP ALL")
        .await?;
    let proj_counts: Vec<serde_json::Value> = response.take(0)?;
    let total_projects = proj_counts.first()
        .and_then(|v| v.get("count"))
        .and_then(|c| c.as_u64())
        .unwrap_or(0);

    let mut response = client.db
        .query("SELECT count() AS count FROM component GROUP ALL")
        .await?;
    let comp_counts: Vec<serde_json::Value> = response.take(0)?;
    let total_components = comp_counts.first()
        .and_then(|v| v.get("count"))
        .and_then(|c| c.as_u64())
        .unwrap_or(0);

    let mut response = client.db
        .query("SELECT count(success = true) AS success, count() AS total FROM compilation GROUP ALL")
        .await?;
    let comp_stats: Vec<serde_json::Value> = response.take(0)?;
    let success = comp_stats.first()
        .and_then(|v| v.get("success"))
        .and_then(|c| c.as_f64())
        .unwrap_or(0.0);
    let total = comp_stats.first()
        .and_then(|v| v.get("total"))
        .and_then(|c| c.as_f64())
        .unwrap_or(0.0);

    let compilation_success_rate = if total > 0.0 { success / total } else { 1.0 };

    Ok(ProjectStats {
        total_projects,
        total_components,
        compilation_success_rate,
    })
}
