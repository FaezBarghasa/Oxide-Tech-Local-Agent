use crate::client::QdrantServiceClient;

pub struct CodeSnippetsCollection<'a> {
    pub service: &'a QdrantServiceClient,
}

impl<'a> CodeSnippetsCollection<'a> {
    pub fn new(service: &'a QdrantServiceClient) -> Self {
        Self { service }
    }
}
