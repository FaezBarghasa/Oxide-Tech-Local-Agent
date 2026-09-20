use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb_types::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
    Published,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogPost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub title_fa: String,
    pub title_en: String,
    pub body_fa: String,
    pub summary_fa: String,
    pub original_links: Vec<String>,
    pub tags: Vec<String>,
    pub published_at: DateTime<Utc>,
    pub status: PostStatus,
}
