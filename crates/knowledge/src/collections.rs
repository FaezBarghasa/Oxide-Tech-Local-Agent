use qdrant_client::Qdrant;
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};
use tracing::info;

pub const COLLECTIONS: &[&str] = &[
    "code",
    "embedded",
    "pcb",
    "schematics",
    "3d",
    "documentation",
    "datasheets",
    "news_raw",
    "blog_posts",
];

pub async fn setup_qdrant_collections(client: &Qdrant) -> Result<(), anyhow::Error> {
    let collections_resp = client.list_collections().await?;
    let existing_names: std::collections::HashSet<String> = collections_resp
        .collections
        .into_iter()
        .map(|c| c.name)
        .collect();

    for col in COLLECTIONS {
        if !existing_names.contains(*col) {
            info!("Creating Qdrant collection: {}", col);
            // Defaulting to 384 dims for BGE-Small-EN-v1.5 embeddings
            client
                .create_collection(
                    CreateCollectionBuilder::new(*col)
                        .vectors_config(VectorParamsBuilder::new(384, Distance::Cosine)),
                )
                .await?;
        }
    }
    Ok(())
}
