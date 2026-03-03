use gitlab::{
    RestError,
    api::{self, Client, Query as _},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: u64,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub default_branch: String,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
}

pub fn fetch_project<C: Client<Error = RestError>>(
    client: &C,
    project: &str,
) -> anyhow::Result<Project> {
    let call = api::projects::Project::builder().project(project).build()?;

    let response: serde_json::Value = call.query(client)?;
    log::trace!("response: {response:#?}");
    let project = serde_json::from_value::<Project>(response)?;

    Ok(project)
}
