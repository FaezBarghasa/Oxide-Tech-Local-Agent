use crate::client::QdrantServiceClient;
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, SearchPoints, VectorParamsBuilder};

pub struct DatasheetCollection<'a> {
    pub service: &'a QdrantServiceClient,
}

impl<'a> DatasheetCollection<'a> {
    pub fn new(service: &'a QdrantServiceClient) -> Self {
        Self { service }
    }

    pub async fn setup_collection(&self) -> Result<(), anyhow::Error> {
        let collection_name = "datasheets";

        let collections = self.service.client.list_collections().await?;
        let exists = collections
            .collections
            .iter()
            .any(|c| c.name == collection_name);
        if exists {
            return Ok(());
        }

        self.service
            .client
            .create_collection(
                CreateCollectionBuilder::new(collection_name)
                    .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine)),
            )
            .await?;

        Ok(())
    }

    pub async fn search_datasheets(
        &self,
        dense_vec: Vec<f32>,
        _mcu_type: &str,
        top_k: usize,
    ) -> Result<Vec<serde_json::Value>, anyhow::Error> {
        let response = self
            .service
            .client
            .search_points(SearchPoints {
                collection_name: "datasheets".to_string(),
                vector: dense_vec,
                limit: top_k as u64,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await?;

        let mut results = Vec::new();
        for point in response.result {
            let payload = serde_json::to_value(&point.payload)?;
            results.push(payload);
        }
        Ok(results)
    }
}
