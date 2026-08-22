use crate::models::BlogPost;
use common::config::BlogConfig;

fn base_template(title: &str, content: &str, config: &BlogConfig) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="fa" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} | {site_name}</title>
    <meta name="description" content="{description}">
    <!-- Google Fonts Vazirmatn -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Vazirmatn:wght@300;400;700&display=swap" rel="stylesheet">
    <style>
        :root {{
            --bg-color: #0a0e14;
            --panel-bg: rgba(20, 26, 38, 0.5);
            --border-color: rgba(255, 255, 255, 0.08);
            --text-primary: #e2e8f0;
            --text-secondary: #94a3b8;
            --accent: #ce422b;
            --accent-hover: #ff5733;
            --accent-glow: rgba(206, 66, 43, 0.2);
            --container-width: 850px;
        }}

        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}

        body {{
            font-family: 'Vazirmatn', sans-serif;
            background-color: var(--bg-color);
            color: var(--text-primary);
            direction: rtl;
            text-align: right;
            line-height: 1.8;
            padding-bottom: 4rem;
        }}

        header {{
            background: rgba(10, 14, 20, 0.7);
            backdrop-filter: blur(12px);
            border-bottom: 1px solid var(--border-color);
            position: sticky;
            top: 0;
            z-index: 100;
        }}

        .header-content {{
            max-width: var(--container-width);
            margin: 0 auto;
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 1rem 1.5rem;
        }}

        .site-title {{
            font-size: 1.4rem;
            font-weight: 700;
            color: var(--text-primary);
            text-decoration: none;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }}

        .site-title span {{
            color: var(--accent);
        }}

        .nav-links {{
            display: flex;
            gap: 1.5rem;
        }}

        .nav-links a {{
            color: var(--text-secondary);
            text-decoration: none;
            font-size: 0.95rem;
            transition: color 0.2s ease;
        }}

        .nav-links a:hover {{
            color: var(--accent);
        }}

        main {{
            max-width: var(--container-width);
            margin: 2.5rem auto 0;
            padding: 0 1.5rem;
        }}

        /* Glassmorphism Card style */
        .card {{
            background: var(--panel-bg);
            backdrop-filter: blur(16px);
            border: 1px solid var(--border-color);
            border-radius: 16px;
            padding: 2rem;
            margin-bottom: 2rem;
            box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);
            transition: transform 0.2s ease, box-shadow 0.2s ease, border-color 0.2s ease;
        }}

        .card:hover {{
            transform: translateY(-4px);
            box-shadow: 0 12px 40px var(--accent-glow);
            border-color: rgba(206, 66, 43, 0.4);
        }}

        .post-title {{
            font-size: 1.8rem;
            font-weight: 700;
            margin-bottom: 0.8rem;
            line-height: 1.4;
        }}

        .post-title a {{
            color: var(--text-primary);
            text-decoration: none;
            transition: color 0.2s ease;
        }}

        .post-title a:hover {{
            color: var(--accent);
        }}

        .meta {{
            display: flex;
            flex-wrap: wrap;
            gap: 1rem;
            font-size: 0.85rem;
            color: var(--text-secondary);
            margin-bottom: 1.5rem;
            border-bottom: 1px solid var(--border-color);
            padding-bottom: 0.8rem;
        }}

        .meta a {{
            color: var(--text-secondary);
            text-decoration: none;
        }}

        .meta a:hover {{
            color: var(--accent);
        }}

        .tag-badge {{
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid var(--border-color);
            color: var(--text-secondary);
            padding: 0.1rem 0.6rem;
            border-radius: 20px;
            text-decoration: none;
            font-size: 0.8rem;
            transition: all 0.2s ease;
        }}

        .tag-badge:hover {{
            background: var(--accent);
            border-color: var(--accent);
            color: #fff;
        }}

        .content {{
            font-size: 1.05rem;
            color: var(--text-primary);
            line-height: 1.9;
        }}

        .content p {{
            margin-bottom: 1.2rem;
        }}

        .content h2, .content h3 {{
            margin: 2rem 0 1rem;
            color: #fff;
        }}

        .content ul, .content ol {{
            margin: 0 1.5rem 1.2rem 0;
        }}

        .content li {{
            margin-bottom: 0.4rem;
        }}

        .footer-content {{
            max-width: var(--container-width);
            margin: 4rem auto 0;
            padding: 2rem 1.5rem 0;
            border-top: 1px solid var(--border-color);
            text-align: center;
            color: var(--text-secondary);
            font-size: 0.9rem;
        }}

        .pagination {{
            display: flex;
            justify-content: center;
            align-items: center;
            gap: 1rem;
            margin-top: 3rem;
        }}

        .btn {{
            background: var(--panel-bg);
            border: 1px solid var(--border-color);
            color: var(--text-primary);
            padding: 0.5rem 1.2rem;
            border-radius: 8px;
            text-decoration: none;
            transition: all 0.2s ease;
            font-size: 0.9rem;
        }}

        .btn:hover {{
            background: var(--accent);
            border-color: var(--accent);
        }}

        /* Code highlight within content */
        pre {{
            background: rgba(0, 0, 0, 0.4);
            border: 1px solid var(--border-color);
            border-radius: 10px;
            padding: 1rem;
            overflow-x: auto;
            margin-bottom: 1.5rem;
            direction: ltr;
            text-align: left;
        }}

        code {{
            font-family: monospace;
            font-size: 0.95rem;
        }}
    </style>
</head>
<body>
    <header>
        <div class="header-content">
            <a href="{base_path}" class="site-title">
                <span>⚡</span> {site_name}
            </a>
            <nav class="nav-links">
                <a href="{base_path}">خانه</a>
                <a href="{base_path}/feed.xml">خوراک RSS</a>
            </nav>
        </div>
    </header>

    <main>
        {content}
    </main>

    <footer>
        <div class="footer-content">
            <p>© {current_year} {site_name}. تمام حقوق محفوظ است.</p>
            <p style="margin-top: 0.5rem; font-size: 0.8rem;">تولید شده به صورت خودکار توسط سیستم هوشمند EIOS</p>
        </div>
    </footer>
</body>
</html>"#,
        title = title,
        site_name = config.site_name,
        description = config.description,
        base_path = config.base_path,
        content = content,
        current_year = chrono::Utc::now().format("%Y")
    )
}

pub fn render_index(
    posts: &[BlogPost],
    page: usize,
    total_pages: usize,
    config: &BlogConfig,
) -> String {
    let mut html = String::new();

    if posts.is_empty() {
        html.push_str(r#"<div class="card" style="text-align: center; padding: 4rem 2rem;">
            <h2 style="margin-bottom: 1rem;">هنوز مقاله‌ای منتشر نشده است</h2>
            <p style="color: var(--text-secondary);">سیستم در حال جمع‌آوری اخبار و تولید مقالات جدید است. لطفاً بعداً سر بزنید.</p>
        </div>"#);
    } else {
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
            let date_str = post.published_at.format("%Y-%m-%d").to_string();

            let tags_html: String = post
                .tags
                .iter()
                .map(|tag| {
                    format!(
                        r#"<a href="{}/tag/{}" class="tag-badge">#{}</a>"#,
                        config.base_path, tag, tag
                    )
                })
                .collect::<Vec<String>>()
                .join(" ");

            html.push_str(&format!(
                r#"<article class="card">
                    <h2 class="post-title"><a href="{}/post/{}">{}</a></h2>
                    <div class="meta">
                        <span>📅 {}</span>
                        <span>🌐 {}</span>
                        <div style="margin-right: auto;">
                            {}
                        </div>
                    </div>
                    <p style="color: var(--text-secondary); margin-bottom: 1.5rem;">{}</p>
                    <a href="{}/post/{}" class="btn" style="display: inline-block;">ادامه مطلب ←</a>
                </article>"#,
                config.base_path,
                post_id,
                post.title_fa,
                date_str,
                post.title_en,
                tags_html,
                post.summary_fa,
                config.base_path,
                post_id
            ));
        }

        // Pagination buttons
        if total_pages > 1 {
            html.push_str(r#"<div class="pagination">"#);
            if page > 1 {
                html.push_str(&format!(
                    r#"<a href="{}/?page={}" class="btn">صفحه قبلی</a>"#,
                    config.base_path,
                    page - 1
                ));
            }
            html.push_str(&format!(
                r#"<span style="color: var(--text-secondary);">صفحه {} از {}</span>"#,
                page, total_pages
            ));
            if page < total_pages {
                html.push_str(&format!(
                    r#"<a href="{}/?page={}" class="btn">صفحه بعدی</a>"#,
                    config.base_path,
                    page + 1
                ));
            }
            html.push_str(r#"</div>"#);
        }
    }

    base_template("صفحه اصلی", &html, config)
}

pub fn render_post(post: &BlogPost, config: &BlogConfig) -> String {
    let date_str = post.published_at.format("%Y-%m-%d").to_string();
    let tags_html: String = post
        .tags
        .iter()
        .map(|tag| {
            format!(
                r#"<a href="{}/tag/{}" class="tag-badge">#{}</a>"#,
                config.base_path, tag, tag
            )
        })
        .collect::<Vec<String>>()
        .join(" ");

    let sources_html = if post.original_links.is_empty() {
        String::new()
    } else {
        let links: String = post
            .original_links
            .iter()
            .enumerate()
            .map(|(i, link)| {
                format!(
                    r#"<li><a href="{}" target="_blank">منبع شماره {}</a></li>"#,
                    link,
                    i + 1
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            r#"<div style="margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid var(--border-color);">
            <h4 style="margin-bottom: 0.8rem; color: #fff;">منابع اصلی خبر:</h4>
            <ul style="list-style-type: none; padding-right: 0;">{}</ul>
        </div>"#,
            links
        )
    };

    let html = format!(
        r#"<article class="card">
            <h1 style="font-size: 2.2rem; font-weight: 700; margin-bottom: 1rem; line-height: 1.4;">{}</h1>
            <div class="meta" style="margin-bottom: 2rem;">
                <span>📅 {}</span>
                <span>🌐 {}</span>
                <div style="margin-right: auto;">
                    {}
                </div>
            </div>
            <div class="content">
                {}
            </div>
            {}
        </article>
        <div style="text-align: center; margin-top: 2rem;">
            <a href="{}" class="btn">← بازگشت به صفحه اصلی</a>
        </div>"#,
        post.title_fa,
        date_str,
        post.title_en,
        tags_html,
        post.body_fa,
        sources_html,
        config.base_path
    );

    base_template(&post.title_fa, &html, config)
}

pub fn render_tag_page(tag: &str, posts: &[BlogPost], config: &BlogConfig) -> String {
    let mut html = format!(
        r#"<div style="margin-bottom: 2.5rem; text-align: center;">
        <h2 style="font-weight: 400;">نمایش مقالات مربوط به برچسب <span style="color: var(--accent); font-weight: 700;">#{}</span></h2>
    </div>"#,
        tag
    );

    if posts.is_empty() {
        html.push_str(
            r#"<div class="card" style="text-align: center; padding: 4rem 2rem;">
            <p style="color: var(--text-secondary);">هیچ مقاله‌ای با این برچسب پیدا نشد.</p>
        </div>"#,
        );
    } else {
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
            let date_str = post.published_at.format("%Y-%m-%d").to_string();

            let tags_html: String = post
                .tags
                .iter()
                .map(|tag| {
                    format!(
                        r#"<a href="{}/tag/{}" class="tag-badge">#{}</a>"#,
                        config.base_path, tag, tag
                    )
                })
                .collect::<Vec<String>>()
                .join(" ");

            html.push_str(&format!(
                r#"<article class="card">
                    <h2 class="post-title"><a href="{}/post/{}">{}</a></h2>
                    <div class="meta">
                        <span>📅 {}</span>
                        <span>🌐 {}</span>
                        <div style="margin-right: auto;">
                            {}
                        </div>
                    </div>
                    <p style="color: var(--text-secondary); margin-bottom: 1.5rem;">{}</p>
                    <a href="{}/post/{}" class="btn" style="display: inline-block;">ادامه مطلب ←</a>
                </article>"#,
                config.base_path,
                post_id,
                post.title_fa,
                date_str,
                post.title_en,
                tags_html,
                post.summary_fa,
                config.base_path,
                post_id
            ));
        }
    }

    base_template(&format!("برچسب #{}", tag), &html, config)
}
