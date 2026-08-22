use select::document::Document;
use select::predicate::Name;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeedEntry {
    pub title: String,
    pub link: String,
    pub summary: String,
    pub published: String,
}

pub fn parse_feed(content: &str) -> Vec<FeedEntry> {
    let doc = Document::from(content);
    let mut entries = Vec::new();

    // Check if it looks like an Atom feed
    let is_atom =
        doc.find(Name("feed")).next().is_some() || doc.find(Name("entry")).next().is_some();

    if is_atom {
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
        // RSS feed
        for item in doc.find(Name("item")) {
            let title = item
                .find(Name("title"))
                .next()
                .map(|n| n.text())
                .unwrap_or_default()
                .trim()
                .to_string();
            let link = item
                .find(Name("link"))
                .next()
                .map(|n| n.text())
                .unwrap_or_default()
                .trim()
                .to_string();

            let summary = item
                .find(Name("description"))
                .next()
                .or_else(|| item.find(Name("content")).next())
                .map(|n| n.text())
                .unwrap_or_default()
                .trim()
                .to_string();

            let published = item
                .find(Name("pubdate"))
                .next()
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
    }

    entries
}
