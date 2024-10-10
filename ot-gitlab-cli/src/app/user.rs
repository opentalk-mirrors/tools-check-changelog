use gitlab::Gitlab;
use secrecy::ExposeSecret as _;

use crate::{
    args::{GitLabApiConfig, OutputFormat, UserCommand},
    gitlab_api::user::current_user,
};

pub(crate) fn run(user_command: UserCommand) -> anyhow::Result<()> {
    match user_command {
        UserCommand::WhoAmI { api, format } => who_am_i(api, format)?,
    }

    Ok(())
}

fn who_am_i(api: GitLabApiConfig, format: OutputFormat) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let me = current_user(&client)?;

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&me)?),
    }
    Ok(())
}
