use crate::client::QdrantServiceClient;

pub struct ComponentLibraryCollection<'a> {
    pub service: &'a QdrantServiceClient,
}

impl<'a> ComponentLibraryCollection<'a> {
    pub fn new(service: &'a QdrantServiceClient) -> Self {
        Self { service }
    }
}
