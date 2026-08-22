use tracing::{info, warn, error};
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use crate::RagPipeline;

#[derive(Debug, Deserialize)]
struct CratesIoCrate {
    max_stable_version: Option<String>,
    max_version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CratesIoResponse {
    #[serde(rename = "crate")]
    krate: CratesIoCrate,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
pub struct CrateVersionRecord {
    pub version: String,
    pub updated_at: String,
}

/// Fetch the latest max stable version of a crate from the crates.io API.
pub async fn fetch_latest_crates_io_version(
    client: &reqwest::Client,
    crate_name: &str,
) -> Result<String, anyhow::Error> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let resp = client.get(&url)
        .header("User-Agent", "Oxide-Tech-Local-Agent/0.1.0 (contact: info@oxide.tech)")
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("crates.io API error for {}: status {}", crate_name, resp.status());
    }

    let parsed: CratesIoResponse = resp.json().await?;
    let version = parsed.krate.max_stable_version
        .or(parsed.krate.max_version)
        .ok_or_else(|| anyhow::anyhow!("No version found for crate {}", crate_name))?;

    Ok(version)
}

/// Iterates through the watchlist of crates. If a crate is missing from SurrealDB
/// or has a newer version on crates.io, we fetch and index its docs.rs pages.
pub async fn check_and_update_crates(
    watchlist: &[String],
    pipeline: &RagPipeline,
) -> Result<(), anyhow::Error> {
    let db = &pipeline.surreal;
    info!("Running RAG watchlist update check...");
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    for crate_name in watchlist {
        info!("Checking crate version for: {}", crate_name);

        let latest_version = match fetch_latest_crates_io_version(&http_client, crate_name).await {
            Ok(ver) => ver,
            Err(e) => {
                warn!("Failed to fetch crates.io version for {}: {}", crate_name, e);
                continue;
            }
        };

        // Query SurrealDB for stored version using raw SQL and owned String bindings
        let existing: Option<CrateVersionRecord> = match db.db
            .query("SELECT version, updated_at FROM type::thing('crate_version', $name)")
            .bind(("name", crate_name.to_string()))
            .await
        {
            Ok(mut resp) => resp.take(0).unwrap_or(None),
            Err(e) => {
                error!("SurrealDB error fetching version for {}: {}", crate_name, e);
                None
            }
        };

        let needs_update = match existing {
            Some(ref rec) => {
                if rec.version != latest_version {
                    info!(
                        "Crate {} has a newer version (stored: {}, crates.io: {}) — updating",
                        crate_name, rec.version, latest_version
                    );
                    true
                } else {
                    info!("Crate {} is up to date (version: {})", crate_name, latest_version);
                    false
                }
            }
            None => {
                info!("Crate {} not found in stored versions — downloading first-time", crate_name);
                true
            }
        };

        if needs_update {
            match pipeline.ingest_crate_docs(crate_name, &latest_version).await {
                Ok(()) => {
                    // Update stored version in SurrealDB using raw SQL and owned String bindings
                    let query_res = db.db
                        .query("UPSERT type::thing('crate_version', $name) SET version = $version, updated_at = $updated_at")
                        .bind(("name", crate_name.to_string()))
                        .bind(("version", latest_version.to_string()))
                        .bind(("updated_at", chrono::Utc::now().to_rfc3339()))
                        .await;

                    if let Err(e) = query_res {
                        error!("Failed to save updated crate version for {} in SurrealDB: {}", crate_name, e);
                    }
                }
                Err(e) => {
                    error!("Failed to ingest docs for {}: {}", crate_name, e);
                }
            }
        }
    }

    info!("RAG watchlist update check complete.");
    Ok(())
}
