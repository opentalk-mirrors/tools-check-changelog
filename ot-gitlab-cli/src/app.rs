use std::path::Path;

use anyhow::Context;
use gitlab::Gitlab;
use secrecy::ExposeSecret;

use crate::{
    args::{AppArgs, DiscussionCommand, GitLabApiConfig, GitLabMrReference, OutputFormat},
    gitlab_api::{
        discussion::{
            create_discussion, fetch_discussion_latest_discussion_by_user, fetch_discussions,
            modify_discussion,
        },
        users::current_user,
    },
    input::read_input,
};

pub fn run(args: AppArgs) -> anyhow::Result<()> {
    match args.command {
        DiscussionCommand::Create { api, mr, body } => discussion_create(api, mr, &body),
        DiscussionCommand::Update { api, mr, id, body } => discussion_update(api, mr, id, &body),
        DiscussionCommand::List { api, mr, format } => discussion_list(api, mr, format),
        DiscussionCommand::UpdateLatest { api, mr, body } => {
            discussion_update_latest(api, mr, &body)
        }
    }
}

fn discussion_list(
    api: GitLabApiConfig,
    mr: GitLabMrReference,
    format: OutputFormat,
) -> anyhow::Result<()> {
    log::info!("List discussions");
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let discussions = fetch_discussions(&client, mr.project(), mr.merge_request_id())?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&discussions)?),
    }

    Ok(())
}

fn discussion_create(
    api: GitLabApiConfig,
    mr: GitLabMrReference,
    body: &Path,
) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let body = read_input(body)?;
    create_discussion(&client, mr.project(), mr.merge_request_id(), &body)?;
    log::info!("discussion created");

    Ok(())
}

fn discussion_update(
    api: GitLabApiConfig,
    mr: GitLabMrReference,
    id: u64,
    body: &Path,
) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let body = read_input(body)?;

    modify_discussion(&client, mr.project(), mr.merge_request_id(), id, &body)?;
    log::info!("discussion updated");
    Ok(())
}

fn discussion_update_latest(
    api: GitLabApiConfig,
    mr: GitLabMrReference,
    body: &Path,
) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    let current_user = current_user(&client)?;
    let body = read_input(body)?;

    let discussion = fetch_discussion_latest_discussion_by_user(
        &client,
        mr.project(),
        mr.merge_request_id(),
        current_user.id,
    )?
    .with_context(|| format!("You didn't open a discussion for merge request {}", mr))?;

    let note = discussion
        .notes
        .first()
        .context("Internal Error, empty discussion?")?;

    modify_discussion(&client, mr.project(), mr.merge_request_id(), note.id, &body)?;

    log::info!("discussion updated");
    Ok(())
}
