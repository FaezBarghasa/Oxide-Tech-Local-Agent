use crate::client::SurrealClient;
use crate::schema::Component;
use surrealdb::Error;

pub async fn create_component(
    client: &SurrealClient,
    ref_des: &str,
    value: &str,
    footprint: &str,
) -> Result<Component, Error> {
    let component = Component {
        id: None,
        ref_des: ref_des.to_string(),
        value: value.to_string(),
        footprint: footprint.to_string(),
        created_at: chrono::Utc::now(),
    };

    let created: Option<Component> = client.db
        .create("component")
        .content(component)
        .await?;
        
    created.ok_or_else(|| Error::thrown("Failed to create component".to_string()))
}

pub async fn get_component_by_ref(
    client: &SurrealClient,
    ref_des: &str,
) -> Result<Option<Component>, Error> {
    let mut response = client.db
        .query("SELECT * FROM component WHERE ref_des = $ref_des")
        .bind(("ref_des", ref_des))
        .await?;
    let mut components: Vec<Component> = response.take(0)?;
    Ok(components.pop())
}

pub async fn list_components(
    client: &SurrealClient,
) -> Result<Vec<Component>, Error> {
    let mut response = client.db
        .query("SELECT * FROM component")
        .await?;
    let components: Vec<Component> = response.take(0)?;
    Ok(components)
}
