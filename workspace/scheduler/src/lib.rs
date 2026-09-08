use chrono::Utc;
use std::time::Duration;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use tracing::{error, info, warn};

pub mod cloud_budget;
pub mod gpu_resource_manager;
pub mod inbox;

pub use inbox::{HitlInboxManager, InboxEntry, InboxStatus};

use blog::BlogPost;
use common::config::AppConfig;
use knowledge::KnowledgeClient;
use thinker::ThinkerClient;

const CURATION_SYSTEM_PROMPT: &str = r#"
You are the Chief Editor for the "Rust Iran Community" (انجمن راست ایران) blog.
Your job is to read a list of raw news articles, release notes, and updates, select the top 3-5 most interesting or important ones, and write a high-quality blog post in Farsi for each.

For each blog post, you must:
1. Translate the title to Farsi ("title_fa").
2. Retain the original English title ("title_en").
3. Write a compelling summary in Farsi ("summary_fa").
4. Write a detailed, educational, and professionally written body in Farsi ("body_fa") formatted in markdown. The markdown should use clean headers, lists, and code blocks where appropriate. Do not include any HTML.
5. Provide relevant tags ("tags") in English (e.g. "async", "embedded", "release").
6. Provide the original URL link ("original_links").

You MUST respond ONLY with a valid JSON array of objects. Do not include markdown code fences, formatting, or explanations outside the JSON block. The JSON must match the following schema:
[
  {
    "title_fa": "...",
    "title_en": "...",
    "body_fa": "...",
    "summary_fa": "...",
    "tags": ["tag1", "tag2"],
    "original_links": ["https://url1"]
  }
]
"#;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RawNewsItem {
    pub title: String,
    pub url: String,
    pub summary: String,
    pub source: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CuratedPostInput {
    pub title_fa: String,
    pub title_en: String,
    pub body_fa: String,
    pub summary_fa: String,
    pub tags: Vec<String>,
    pub original_links: Vec<String>,
}

pub fn start_scheduler(config: AppConfig, db: Surreal<Any>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        info!("EIOS Scheduler background task started.");

        let interval_hours = config.knowledge.update_interval_hours;
        // Check for updates periodically
        let mut interval = tokio::time::interval(Duration::from_secs(interval_hours * 3600));

        loop {
            interval.tick().await;
            info!("Starting scheduled daily knowledge update and blog curation...");

            if let Err(e) = run_update_cycle(&config, &db).await {
                error!("Error in scheduled update cycle: {}", e);
            }

            info!(
                "Scheduled daily cycle completed. Sleeping for {} hours.",
                interval_hours
            );
        }
    })
}

pub async fn run_update_cycle(config: &AppConfig, db: &Surreal<Any>) -> Result<(), anyhow::Error> {
    info!("Initializing KnowledgeClient and ThinkerClient for update cycle...");
    let knowledge_client = KnowledgeClient::new().await?;
    let thinker_client = ThinkerClient::new();

    let mut raw_news = Vec::new();

    // 1. Ingest RSS feeds
    for (name, url) in &config.knowledge.news_sources {
        info!("Ingesting news source: {} from {}", name, url);
        if let Err(e) = knowledge_client.ingest_rss_feed(url, name).await {
            warn!("Failed to ingest RSS feed {}: {}", name, e);
        }

        // Also fetch and parse locally for curation
        if let Ok(resp) = reqwest::get(url).await {
            if let Ok(body) = resp.text().await {
                let entries = knowledge::parse_feed(&body);
                for entry in entries {
                    raw_news.push(RawNewsItem {
                        title: entry.title,
                        url: entry.link,
                        summary: entry.summary,
                        source: name.clone(),
                    });
                }
            }
        }
    }

    // 2. Ingest GitHub repos
    for repo in &config.knowledge.github_repos {
        info!("Ingesting GitHub repo: {}", repo);
        if let Err(e) = knowledge_client.ingest_github_releases(repo).await {
            warn!("Failed to ingest GitHub repo {}: {}", repo, e);
        }

        let clean_url = repo.trim_end_matches('/');
        if clean_url.contains("github.com/") {
            let parts: Vec<&str> = clean_url.split("github.com/").collect();
            if parts.len() >= 2 {
                let path = parts[1];
                let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
                if segments.len() >= 2 {
                    let owner = segments[0];
                    let repo_name = segments[1];
                    let feed_url =
                        format!("https://github.com/{}/{}/releases.atom", owner, repo_name);

                    if let Ok(resp) = reqwest::get(&feed_url).await {
                        if let Ok(body) = resp.text().await {
                            let entries = knowledge::parse_feed(&body);
                            for entry in entries {
                                raw_news.push(RawNewsItem {
                                    title: format!("{} Release: {}", repo_name, entry.title),
                                    url: entry.link,
                                    summary: entry.summary,
                                    source: "GitHub".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Ingest custom URLs
    for url in &config.knowledge.custom_urls {
        info!("Ingesting custom URL: {}", url);
        if let Err(e) = knowledge_client.ingest_custom_url(url).await {
            warn!("Failed to ingest custom URL {}: {}", url, e);
        }
    }

    // 4. Ingest watchlist crate docs.rs
    for crate_name in &config.knowledge.watchlist {
        info!("Ingesting docs for crate: {}", crate_name);
        if let Err(e) = knowledge_client
            .ingest_crate_docs(crate_name, "latest")
            .await
        {
            warn!("Failed to ingest crate docs for {}: {}", crate_name, e);
        }
        tokio::time::sleep(Duration::from_millis(config.knowledge.request_delay_ms)).await;
    }

    // 5. Curation and translation via Thinker LLM
    if !raw_news.is_empty() {
        info!(
            "Collected {} raw news items. Running curation pipeline...",
            raw_news.len()
        );
        if let Err(e) = curate_and_publish_news(config, db, &thinker_client, raw_news).await {
            error!("Curation pipeline failed: {}", e);
        }
    } else {
        info!("No new news items discovered during this update cycle.");
    }

    Ok(())
}

async fn curate_and_publish_news(
    config: &AppConfig,
    db: &Surreal<Any>,
    thinker_client: &ThinkerClient,
    items: Vec<RawNewsItem>,
) -> Result<(), anyhow::Error> {
    let items_to_send: Vec<RawNewsItem> = items.into_iter().take(15).collect();
    let user_prompt = serde_json::to_string_pretty(&items_to_send)?;

    info!("Requesting translation and curation from Thinker LLM...");
    let raw_resp = thinker_client
        .complete_prompt(CURATION_SYSTEM_PROMPT, &user_prompt, config)
        .await?;

    let cleaned = raw_resp
        .trim()
        .strip_prefix("```json")
        .unwrap_or(&raw_resp)
        .strip_prefix("```")
        .unwrap_or(&raw_resp)
        .strip_suffix("```")
        .unwrap_or(&raw_resp)
        .trim()
        .to_string();

    let curated_posts: Vec<CuratedPostInput> = serde_json::from_str(&cleaned).map_err(|e| {
        anyhow::anyhow!(
            "Failed to parse LLM curation JSON: {}. Raw response: {}",
            e,
            raw_resp
        )
    })?;

    info!("Thinker curated {} blog posts.", curated_posts.len());

    for post in curated_posts {
        let blog_post = BlogPost {
            id: None,
            title_fa: post.title_fa,
            title_en: post.title_en,
            body_fa: post.body_fa,
            summary_fa: post.summary_fa,
            original_links: post.original_links,
            tags: post.tags,
            published_at: Utc::now(),
            status: if config.blog.auto_publish {
                blog::PostStatus::Published
            } else {
                blog::PostStatus::Draft
            },
        };

        match blog::create_post(db, blog_post).await {
            Ok(created) => {
                info!("Successfully published blog post: {}", created.title_en);
            }
            Err(e) => {
                error!("Failed to save curated blog post: {}", e);
            }
        }
    }

    Ok(())
}
