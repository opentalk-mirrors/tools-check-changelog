use gitlab::Gitlab;
use secrecy::ExposeSecret as _;

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
    }
}

fn project_get(api: GitLabApiConfig, format: OutputFormat, project: &str) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let project = fetch_project(&client, project)?;

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&project)?),
    }
    Ok(())
}
