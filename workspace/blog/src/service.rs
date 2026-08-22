use crate::models::BlogPost;
use chrono::Utc;
use common::error::{EiosError, Result};
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

pub async fn create_post(db: &Surreal<Any>, post: BlogPost) -> Result<BlogPost> {
    let post_json = serde_json::to_value(&post).map_err(|e| EiosError::Serialization(e))?;

    let created_opt: Option<serde_json::Value> = db
        .create("blog_post")
        .content(post_json)
        .await
        .map_err(|e| EiosError::Database(e.to_string()))?;

    let created_val = created_opt.ok_or_else(|| {
        EiosError::Database("Failed to create blog post: None returned".to_string())
    })?;

    let created: BlogPost =
        serde_json::from_value(created_val).map_err(|e| EiosError::Serialization(e))?;
    Ok(created)
}

pub async fn get_post(db: &Surreal<Any>, id: &str) -> Result<Option<BlogPost>> {
    let record_id = if id.contains(':') {
        id.to_string()
    } else {
        format!("blog_post:{}", id)
    };

    let posts_val: Vec<serde_json::Value> = db
        .select(record_id)
        .await
        .map_err(|e| EiosError::Database(e.to_string()))?;

    if let Some(post_val) = posts_val.into_iter().next() {
        let post: BlogPost =
            serde_json::from_value(post_val).map_err(|e| EiosError::Serialization(e))?;
        Ok(Some(post))
    } else {
        Ok(None)
    }
}

pub async fn list_posts(db: &Surreal<Any>, page: usize, per_page: usize) -> Result<Vec<BlogPost>> {
    let limit = per_page;
    let start = if page > 0 { (page - 1) * per_page } else { 0 };

    let mut response = db.query("SELECT * FROM blog_post WHERE status = 'published' ORDER BY published_at DESC LIMIT $limit START $start")
        .bind(("limit", limit))
        .bind(("start", start))
        .await
        .map_err(|e| EiosError::Database(e.to_string()))?;

    let posts_val: Vec<serde_json::Value> = response
        .take(0)
        .map_err(|e| EiosError::Database(e.to_string()))?;

    let posts: Vec<BlogPost> = serde_json::from_value(serde_json::Value::Array(posts_val))
        .map_err(|e| EiosError::Serialization(e))?;
    Ok(posts)
}

pub async fn list_by_tag(db: &Surreal<Any>, tag: &str) -> Result<Vec<BlogPost>> {
    let mut response = db.query("SELECT * FROM blog_post WHERE status = 'published' AND $tag INSIDE tags ORDER BY published_at DESC")
        .bind(("tag", tag))
        .await
        .map_err(|e| EiosError::Database(e.to_string()))?;

    let posts_val: Vec<serde_json::Value> = response
        .take(0)
        .map_err(|e| EiosError::Database(e.to_string()))?;

    let posts: Vec<BlogPost> = serde_json::from_value(serde_json::Value::Array(posts_val))
        .map_err(|e| EiosError::Serialization(e))?;
    Ok(posts)
}

pub async fn get_rss_feed(
    db: &Surreal<Any>,
    site_name: &str,
    description: &str,
    base_url: &str,
) -> Result<String> {
    let posts = list_posts(db, 1, 20).await?;
    let updated = posts
        .first()
        .map(|p| p.published_at.to_rfc3339())
        .unwrap_or_else(|| Utc::now().to_rfc3339());

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{}</title>
  <subtitle>{}</subtitle>
  <link href="{}/feed.xml" rel="self"/>
  <link href="{}/"/>
  <updated>{}</updated>
  <id>{}</id>
"#,
        escape_xml(site_name),
        escape_xml(description),
        base_url.trim_end_matches('/'),
        base_url.trim_end_matches('/'),
        updated,
        base_url.trim_end_matches('/')
    );

    for post in posts {
        let post_id = post
            .id
            .as_ref()
            .map(|id| match &id.key {
                surrealdb_types::RecordIdKey::String(s) => s.clone(),
                surrealdb_types::RecordIdKey::Number(n) => n.to_string(),
                surrealdb_types::RecordIdKey::Uuid(u) => u.to_string(),
                _ => format!("{:?}", id.key),
            })
            .unwrap_or_default();
        let post_url = format!("{}/post/{}", base_url.trim_end_matches('/'), post_id);
        xml.push_str(&format!(
            r#"  <entry>
    <title>{}</title>
    <link href="{}"/>
    <id>{}</id>
    <updated>{}</updated>
    <summary type="text">{}</summary>
    <content type="html"><![CDATA[{}]]></content>
  </entry>
"#,
            escape_xml(&post.title_fa),
            post_url,
            post_url,
            post.published_at.to_rfc3339(),
            escape_xml(&post.summary_fa),
            post.body_fa
        ));
    }

    xml.push_str("</feed>");
    Ok(xml)
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub async fn count_posts(db: &Surreal<Any>) -> Result<usize> {
    let mut response = db
        .query("SELECT count() FROM blog_post WHERE status = 'published' GROUP ALL")
        .await
        .map_err(|e| EiosError::Database(e.to_string()))?;

    let res_opt: Option<serde_json::Value> = response
        .take(0)
        .map_err(|e| EiosError::Database(e.to_string()))?;

    if let Some(val) = res_opt {
        let count = val.get("count").and_then(|c| c.as_u64()).unwrap_or(0) as usize;
        Ok(count)
    } else {
        Ok(0)
    }
}
