use crate::client::SurrealClient;
use crate::schema::Component;
use surrealdb::Error;

pub async fn get_net_connections(
    client: &SurrealClient,
    net_name: &str,
) -> Result<Vec<Component>, Error> {
    let mut response = client.db
        .query("SELECT ->net_junction->pin->maps_pin->component.* AS component FROM net WHERE name = $netname")
        .bind(("netname", net_name))
        .await?;
    let components: Vec<Component> = response.take(0)?;
    Ok(components)
}
