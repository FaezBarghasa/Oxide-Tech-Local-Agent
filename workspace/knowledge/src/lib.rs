pub mod ast;
pub mod client;
pub mod collections;
pub mod indexer;
pub mod rss;
pub mod code_graph;
pub mod impact_analysis;
pub mod subgraph_pruner;

pub use ast::{ParsedSymbol, AstGraphExtractor, CodeGraphNode, CodeGraphEdge, CodeNodeType as AstCodeNodeType, CodeEdgeType as AstCodeEdgeType};
pub use client::{EiosChunk, KnowledgeClient};
pub use collections::COLLECTIONS;
pub use rss::{FeedEntry, parse_feed};
pub use code_graph::{CodeEdge, CodeEdgeType, CodeNode, CodeNodeType, MultiModalCodeGraph};
pub use impact_analysis::{ImpactAnalysisEngine, ImpactAnalysisReport, ImpactRiskLevel};
pub use subgraph_pruner::{PrunedSubgraph, SubgraphPruner};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom_feed() {
        let atom = r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Test Feed</title>
  <entry>
    <title>Atom Release</title>
    <link href="https://github.com/test/repo/releases/v1.0.0"/>
    <summary>Version 1.0.0 release notes.</summary>
    <published>2026-06-21T12:00:00Z</published>
  </entry>
</feed>"#;

        let entries = parse_feed(atom);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].title, "Atom Release");
        assert_eq!(
            entries[0].link,
            "https://github.com/test/repo/releases/v1.0.0"
        );
        assert_eq!(entries[0].summary, "Version 1.0.0 release notes.");
        assert_eq!(entries[0].published, "2026-06-21T12:00:00Z");
    }

    #[test]
    fn test_parse_rss_feed() {
        let rss = r#"<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0">
  <channel>
    <title>Test RSS</title>
    <item>
      <title>RSS Item</title>
      <link>https://blog.example.com/post/1</link>
      <description>Description of post 1.</description>
      <pubDate>Sun, 21 Jun 2026 12:00:00 GMT</pubDate>
    </item>
  </channel>
</rss>"#;

        let entries = parse_feed(rss);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].title, "RSS Item");
        assert_eq!(entries[0].link, "https://blog.example.com/post/1");
        assert_eq!(entries[0].summary, "Description of post 1.");
        assert_eq!(entries[0].published, "Sun, 21 Jun 2026 12:00:00 GMT");
    }
}
