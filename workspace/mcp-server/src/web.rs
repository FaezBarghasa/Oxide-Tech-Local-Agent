use reqwest::header::USER_AGENT;
use serde_json::Value;

const BROWSER_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// Perform Google Search or DuckDuckGo search fallback, returning text output.
pub async fn google_search(query: &str) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // Try Google first
    let google_url = format!(
        "https://www.google.com/search?q={}",
        urlencoding::encode(query)
    );
    let resp = client
        .get(&google_url)
        .header(USER_AGENT, BROWSER_UA)
        .send()
        .await;

    match resp {
        Ok(res) if res.status().is_success() => {
            if let Ok(html) = res.text().await {
                // Parse google results using regex to find result blocks
                let mut results = Vec::new();
                let re = regex::Regex::new(
                    r#"<a href="/url\?q=([^&"]+)[^>]+><h3[^>]*><div[^>]*>([^<]+)</div></h3>"#,
                )
                .unwrap();
                for cap in re.captures_iter(&html) {
                    let url = urlencoding::decode(&cap[1])
                        .unwrap_or_else(|_| cap[1].into())
                        .to_string();
                    let title = cap[2].to_string();
                    results.push(format!("- **{}**\n  URL: {}", title, url));
                }

                // If regex parsing failed, try alternate layout parsing
                if results.is_empty() {
                    let re_alt = regex::Regex::new(r#"<div class="egMi0 kCrYT"><a href="/url\?q=([^&"]+)[^>]+><span class="XL7Z2c[^>]*>([^<]+)</span>"#).unwrap();
                    for cap in re_alt.captures_iter(&html) {
                        let url = urlencoding::decode(&cap[1])
                            .unwrap_or_else(|_| cap[1].into())
                            .to_string();
                        let title = cap[2].to_string();
                        results.push(format!("- **{}**\n  URL: {}", title, url));
                    }
                }

                if !results.is_empty() {
                    return Ok(format!(
                        "Google Search Results for '{}':\n\n{}",
                        query,
                        results.join("\n\n")
                    ));
                }
            }
        }
        _ => {}
    }

    // Fallback to DuckDuckGo if Google fails or blocks requests
    let ddg_url = format!(
        "https://html.duckduckgo.com/html/?q={}",
        urlencoding::encode(query)
    );
    let resp_ddg = client
        .get(&ddg_url)
        .header(USER_AGENT, BROWSER_UA)
        .send()
        .await;

    match resp_ddg {
        Ok(res) if res.status().is_success() => {
            if let Ok(html) = res.text().await {
                let mut results = Vec::new();
                let re = regex::Regex::new(r#"<a class="result__url" href="([^"]+)">"#).unwrap();
                let re_title =
                    regex::Regex::new(r#"<a class="result__snippet"[^>]*>([^<]+)</a>"#).unwrap();

                let urls: Vec<String> = re.captures_iter(&html).map(|c| c[1].to_string()).collect();
                let snippets: Vec<String> = re_title
                    .captures_iter(&html)
                    .map(|c| c[1].to_string())
                    .collect();

                for (url, snippet) in urls.into_iter().zip(snippets.into_iter()) {
                    results.push(format!("- URL: {}\n  Snippet: {}", url, snippet));
                }

                if !results.is_empty() {
                    return Ok(format!(
                        "DuckDuckGo Search Results (Google fallback) for '{}':\n\n{}",
                        query,
                        results.join("\n\n")
                    ));
                }
            }
        }
        _ => {}
    }

    Err(format!(
        "Search failed for query '{}'. Google and DuckDuckGo returned no parsed results or blocked the scraping request.",
        query
    ))
}

/// Query the latest release tag, name, date, and description for a GitHub repository.
pub async fn github_latest_release(repo: &str) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let api_url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let resp = client
        .get(&api_url)
        .header(USER_AGENT, "EIOS-MCP-Server")
        .send()
        .await
        .map_err(|e| format!("Failed to query GitHub API: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "GitHub API returned error code {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        ));
    }

    let release: Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse GitHub JSON response: {}", e))?;

    let tag = release
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let name = release
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("no name");
    let published = release
        .get("published_at")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let body = release.get("body").and_then(|v| v.as_str()).unwrap_or("");

    Ok(format!(
        "Latest Release for GitHub repository '{}':\n\n**Tag**: {}\n**Name**: {}\n**Date**: {}\n\n**Description**:\n{}",
        repo, tag, name, published, body
    ))
}
