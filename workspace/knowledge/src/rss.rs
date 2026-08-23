use select::document::Document;
use select::predicate::Name;
use regex::Regex;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeedEntry {
    pub title: String,
    pub link: String,
    pub summary: String,
    pub published: String,
}

pub fn parse_feed(content: &str) -> Vec<FeedEntry> {
    let mut entries = Vec::new();

    // Check if it's an Atom feed (<feed> or <entry>)
    if content.contains("<feed") || (content.contains("<entry") && !content.contains("<rss")) {
        let doc = Document::from(content);
        for entry in doc.find(Name("entry")) {
            let title = entry
                .find(Name("title"))
                .next()
                .map(|n| n.text())
                .unwrap_or_default()
                .trim()
                .to_string();
            let link = entry
                .find(Name("link"))
                .next()
                .and_then(|n| {
                    n.attr("href").map(|s| s.to_string()).or_else(|| {
                        let text = n.text().trim().to_string();
                        if !text.is_empty() { Some(text) } else { None }
                    })
                })
                .unwrap_or_default();

            let summary = entry
                .find(Name("summary"))
                .next()
                .or_else(|| entry.find(Name("content")).next())
                .map(|n| n.text())
                .unwrap_or_default()
                .trim()
                .to_string();

            let published = entry
                .find(Name("published"))
                .next()
                .or_else(|| entry.find(Name("updated")).next())
                .map(|n| n.text())
                .unwrap_or_default()
                .trim()
                .to_string();

            if !title.is_empty() && !link.is_empty() {
                entries.push(FeedEntry {
                    title,
                    link,
                    summary,
                    published,
                });
            }
        }
    } else {
        // RSS 2.0 / RDF parsing: extract <item> blocks via regex if HTML parser collapses XML tags
        let item_re = Regex::new(r"(?s)<item>(.*?)</item>").unwrap();
        let title_re = Regex::new(r"(?s)<title>(.*?)</title>").unwrap();
        let link_re = Regex::new(r"(?s)<link>(.*?)</link>").unwrap();
        let desc_re = Regex::new(r"(?s)<description>(.*?)</description>").unwrap();
        let pub_re = Regex::new(r"(?si)<pubdate>(.*?)</pubdate>").unwrap();

        for cap in item_re.captures_iter(content) {
            let block = &cap[1];
            let title = title_re.captures(block).map(|c| c[1].trim().to_string()).unwrap_or_default();
            let link = link_re.captures(block).map(|c| c[1].trim().to_string()).unwrap_or_default();
            let summary = desc_re.captures(block).map(|c| c[1].trim().to_string()).unwrap_or_default();
            let published = pub_re.captures(block).map(|c| c[1].trim().to_string()).unwrap_or_default();

            if !title.is_empty() && !link.is_empty() {
                entries.push(FeedEntry {
                    title,
                    link,
                    summary,
                    published,
                });
            }
        }
    }

    entries
}
