use crate::client::SurrealClient;
use crate::schema::Project;
use surrealdb::Error;

pub async fn create_project(
    client: &SurrealClient,
    name: &str,
    mcu_type: &str,
    target_triple: Option<String>,
) -> Result<Project, Error> {
    let project = Project {
        id: None,
        name: name.to_string(),
        mcu_type: mcu_type.to_string(),
        target_triple,
        created_at: chrono::Utc::now(),
    };

    let created: Option<Project> = client.db
        .create("project")
        .content(project)
        .await?;
        
    created.ok_or_else(|| Error::thrown("Failed to create project".to_string()))
}

pub async fn get_project(
    client: &SurrealClient,
    name: &str,
) -> Result<Option<Project>, Error> {
    let mut response = client.db
        .query("SELECT * FROM project WHERE name = $name")
        .bind(("name", name))
        .await?;
    let mut projects: Vec<Project> = response.take(0)?;
    Ok(projects.pop())
}

pub async fn list_projects(
    client: &SurrealClient,
) -> Result<Vec<Project>, Error> {
    let mut response = client.db
        .query("SELECT * FROM project")
        .await?;
    let projects: Vec<Project> = response.take(0)?;
    Ok(projects)
}
