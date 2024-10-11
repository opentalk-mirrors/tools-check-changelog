use std::str::FromStr as _;

use anyhow::anyhow;
use gitlab::Gitlab;
use secrecy::ExposeSecret as _;
use url::Url;

use crate::{
    args::{GitLabApiConfig, OutputFormat, ProjectCommand},
    gitlab_api::project::fetch_project,
};

pub(crate) fn run(project_command: ProjectCommand) -> anyhow::Result<()> {
    match project_command {
        ProjectCommand::Get {
            api,
            format,
            project,
        } => project_get(api, format, &project),
        ProjectCommand::AuthorizedCloneUrl { api, project } => authorized_clone_url(api, &project),
    }
}

fn authorized_clone_url(api: GitLabApiConfig, project: &str) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let project = fetch_project(&client, project)?;
    let mut clone_url = Url::from_str(&project.http_url_to_repo)?;
    clone_url
        .set_username("oauth2")
        .map_err(|_| anyhow!("Failed to set URL username"))?;
    clone_url
        .set_password(Some(api.token.expose_secret()))
        .map_err(|_| anyhow!("Failed to set URL password"))?;

    println!("{}", clone_url.as_str());

    Ok(())
}

fn project_get(api: GitLabApiConfig, format: OutputFormat, project: &str) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let project = fetch_project(&client, project)?;

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&project)?),
    }
    Ok(())
}
