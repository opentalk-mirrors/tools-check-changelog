use gitlab::Gitlab;
use secrecy::ExposeSecret;

use crate::{
    args::{AppArgs, DiscussionCommand, GitLabApiConfig, GitLabMrReference},
    gitlab_api::discussion::fetch_discussions,
};

pub fn run(args: AppArgs) -> anyhow::Result<()> {
    match args.command {
        DiscussionCommand::Create => todo!(),
        DiscussionCommand::Update => todo!(),
        DiscussionCommand::Delete => todo!(),
        DiscussionCommand::List { api, mr } => discussion_list(api, mr),
    }
}

fn discussion_list(api: GitLabApiConfig, mr: GitLabMrReference) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let discussions = fetch_discussions(&client, mr.project(), mr.merge_request_id())?;
    println!("Discussions: {:#?}", discussions);
    Ok(())
}
