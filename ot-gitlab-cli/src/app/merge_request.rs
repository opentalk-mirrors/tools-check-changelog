use gitlab::Gitlab;
use secrecy::ExposeSecret as _;

use crate::{
    args::{GitLabApiConfig, GitLabMrReference, MergeRequestCommand, OutputFormat},
    gitlab_api,
};

pub(crate) fn run(command: MergeRequestCommand) -> anyhow::Result<()> {
    match command {
        MergeRequestCommand::GetLabels { api, format, mr } => get_label(api, format, mr)?,
    }

    Ok(())
}

fn get_label(
    api: GitLabApiConfig,
    format: OutputFormat,
    mr: GitLabMrReference,
) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let labels = gitlab_api::merge_request::get_label(&client, mr)?;

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&labels)?),
    }

    Ok(())
}
