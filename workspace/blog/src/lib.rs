pub mod models;
pub mod service;
pub mod templates;

pub use models::{BlogPost, PostStatus};
pub use service::{count_posts, create_post, get_post, get_rss_feed, list_by_tag, list_posts};
pub use templates::{render_index, render_post, render_tag_page};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_blog_post_serde() {
        let post = BlogPost {
            id: None,
            title_fa: "تست راست".to_string(),
            title_en: "Rust Test".to_string(),
            body_fa: "<p>سلام دنیا</p>".to_string(),
            summary_fa: "توضیحات تست".to_string(),
            original_links: vec!["https://example.com".to_string()],
            tags: vec!["test".to_string()],
            published_at: Utc::now(),
            status: PostStatus::Published,
        };

        let serialized = serde_json::to_string(&post).unwrap();
        let deserialized: BlogPost = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.title_fa, "تست راست");
        assert_eq!(deserialized.title_en, "Rust Test");
    }

    #[test]
    fn test_blog_html_rendering() {
        let config = common::config::BlogConfig::default();
        let post = BlogPost {
            id: None,
            title_fa: "تست قالب راست".to_string(),
            title_en: "Rust Template Test".to_string(),
            body_fa: "<p>محتوای بدنه</p>".to_string(),
            summary_fa: "خلاصه".to_string(),
            original_links: vec![],
            tags: vec!["rust".to_string()],
            published_at: Utc::now(),
            status: PostStatus::Published,
        };

        let html = render_post(&post, &config);
        assert!(html.contains("تست قالب راست"));
        assert!(html.contains("Vazirmatn"));
        assert!(html.contains("dir=\"rtl\""));
        assert!(html.contains("#rust"));
    }
}
