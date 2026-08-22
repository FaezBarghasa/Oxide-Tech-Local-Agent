use knowledge::KnowledgeClient;
use reqwest::header::USER_AGENT;
use serde_json::Value;
use std::sync::Arc;

/// Lookup docs in the Qdrant RAG client.
pub async fn docs_rs_lookup(
    query: &str,
    limit: Option<usize>,
    rag: &Option<Arc<KnowledgeClient>>,
) -> Result<String, String> {
    if let Some(r) = rag {
        let results = r
            .search("documentation", query, limit.unwrap_or(8))
            .await
            .map_err(|e| format!("RAG Search failed: {}", e))?;

        if results.is_empty() {
            return Ok(format!(
                "No documentation matches found in Qdrant for: {}",
                query
            ));
        }

        let mut out = String::new();
        for (i, chunk) in results.into_iter().enumerate() {
            out.push_str(&format!(
                "Match {}:\nSource: {}\nCrate: {:?} v{:?}\nFile: {:?}\nContent:\n{}\n\n",
                i + 1,
                chunk.source,
                chunk.crate_name,
                chunk.version,
                chunk.file_path,
                chunk.text
            ));
        }
        Ok(out)
    } else {
        Err("Qdrant knowledge client is not initialized".to_string())
    }
}

/// Search Crates.io and return downloads, maintenance score (custom heuristic), and alternatives.
pub async fn crates_io_search(query: &str) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let url = format!(
        "https://crates.io/api/v1/crates?q={}",
        urlencoding::encode(query)
    );
    let resp = client
        .get(&url)
        .header(USER_AGENT, "EIOS-MCP-Server")
        .send()
        .await
        .map_err(|e| format!("Failed to query Crates.io: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Crates.io API returned error code {}",
            resp.status()
        ));
    }

    let results: Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse Crates.io JSON response: {}", e))?;

    let Some(crates) = results.get("crates").and_then(|v| v.as_array()) else {
        return Ok("No crates found matching the search query".to_string());
    };

    let mut out = format!("Crates.io search results for '{}':\n\n", query);
    for (i, c) in crates.iter().enumerate().take(5) {
        let name = c.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
        let desc = c
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("no description");
        let downloads = c.get("downloads").and_then(|v| v.as_u64()).unwrap_or(0);
        let updated = c
            .get("updated_at")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        // Custom maintenance score logic:
        // Score = 100 base. Deduct points for old updates.
        let mut score = 100;
        if let Ok(updated_time) = chrono::DateTime::parse_from_rfc3339(updated) {
            let days_since_update =
                (chrono::Utc::now() - updated_time.with_timezone(&chrono::Utc)).num_days();
            if days_since_update > 365 {
                score -= 30; // Not updated in a year
            } else if days_since_update > 180 {
                score -= 10;
            }
        } else {
            score = 50;
        }

        out.push_str(&format!(
            "{}. **{}**\n   Description: {}\n   Downloads: {}\n   Last Updated: {}\n   Maintenance Score: {}/100\n\n",
            i + 1, name, desc, downloads, updated, score
        ));
    }

    Ok(out)
}

/// Search Rust Book for query.
pub async fn rust_book_search(query: &str) -> Result<String, String> {
    // Perform standard scraping fallback on rust-lang book or return clean reference mapping
    let result =
        crate::web::google_search(&format!("site:doc.rust-lang.org/book/ {}", query)).await?;
    Ok(format!("Rust Book Search results:\n\n{}", result))
}

/// Query specific endpoints for GitHub repository.
pub async fn github_api_query(repo: &str, endpoint: &str) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let api_url = format!("https://api.github.com/repos/{}/{}", repo, endpoint);
    let resp = client
        .get(&api_url)
        .header(USER_AGENT, "EIOS-MCP-Server")
        .send()
        .await
        .map_err(|e| format!("GitHub request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "GitHub API endpoint /{} returned error code {}",
            endpoint,
            resp.status()
        ));
    }

    let data: Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub JSON response: {}", e))?;

    let out = serde_json::to_string_pretty(&data)
        .unwrap_or_else(|_| "Failed to format response".to_string());

    // Return truncated version to avoid huge outputs
    if out.len() > 8000 {
        Ok(format!(
            "{}\n... (truncated to 8000 characters)",
            &out[..8000]
        ))
    } else {
        Ok(out)
    }
}

/// Find changelogs or release details matching breaking changes between crate versions.
pub async fn changelog_diff(
    crate_name: &str,
    from_ver: &str,
    to_ver: &str,
) -> Result<String, String> {
    // Search GitHub release notes or crates.io changelog url
    let query = format!(
        "\"{}\" changelog OR breaking changes \"{}\" to \"{}\"",
        crate_name, from_ver, to_ver
    );
    let result = crate::web::google_search(&query).await?;
    Ok(format!(
        "Changelog search for {} from {} to {}:\n\n{}",
        crate_name, from_ver, to_ver, result
    ))
}
