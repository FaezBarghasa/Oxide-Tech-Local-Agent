use knowledge::KnowledgeClient;
use std::sync::Arc;

/// Queries Embassy documentation, examples, and HAL drivers.
pub async fn embassy_lookup(
    query: &str,
    rag: &Option<Arc<KnowledgeClient>>,
) -> Result<String, String> {
    // 1. Search locally in RAG first if available
    if let Some(r) = rag {
        if let Ok(results) = r
            .search("documentation", &format!("embassy {}", query), 3)
            .await
        {
            if !results.is_empty() {
                let mut out = String::from("Embassy RAG matches found:\n\n");
                for (i, chunk) in results.into_iter().enumerate() {
                    out.push_str(&format!("{}. {}\n\n", i + 1, chunk.text));
                }
                return Ok(out);
            }
        }
    }

    // 2. Fall back to scoped Google/DuckDuckGo search
    let web_query = format!(
        "site:embassy.dev OR github.com/embassy-rs/embassy {}",
        query
    );
    let search_res = crate::web::google_search(&web_query).await?;
    Ok(format!(
        "Embassy Reference Search results:\n\n{}",
        search_res
    ))
}

/// Queries STM32 datasheets, reference manuals, errata, and registers.
pub async fn stm32_lookup(
    query: &str,
    rag: &Option<Arc<KnowledgeClient>>,
) -> Result<String, String> {
    if let Some(r) = rag {
        if let Ok(results) = r
            .search("documentation", &format!("stm32 {}", query), 3)
            .await
        {
            if !results.is_empty() {
                let mut out = String::from("STM32 RAG matches found:\n\n");
                for (i, chunk) in results.into_iter().enumerate() {
                    out.push_str(&format!("{}. {}\n\n", i + 1, chunk.text));
                }
                return Ok(out);
            }
        }
    }

    let web_query = format!("site:st.com \"stm32\" {}", query);
    let search_res = crate::web::google_search(&web_query).await?;
    Ok(format!("STM32 Reference Search results:\n\n{}", search_res))
}

/// Queries ESP32 IDF documentation, examples, and migration guides.
pub async fn esp32_lookup(
    query: &str,
    rag: &Option<Arc<KnowledgeClient>>,
) -> Result<String, String> {
    if let Some(r) = rag {
        if let Ok(results) = r
            .search("documentation", &format!("esp32 {}", query), 3)
            .await
        {
            if !results.is_empty() {
                let mut out = String::from("ESP32 RAG matches found:\n\n");
                for (i, chunk) in results.into_iter().enumerate() {
                    out.push_str(&format!("{}. {}\n\n", i + 1, chunk.text));
                }
                return Ok(out);
            }
        }
    }

    let web_query = format!("site:docs.espressif.com/projects/esp-idf/ {}", query);
    let search_res = crate::web::google_search(&web_query).await?;
    Ok(format!("ESP32 Reference Search results:\n\n{}", search_res))
}
