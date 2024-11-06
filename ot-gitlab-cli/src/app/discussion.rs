use std::path::Path;

use anyhow::Context as _;
use gitlab::Gitlab;
use secrecy::ExposeSecret as _;

use crate::{
    args::{DiscussionCommand, GitLabApiConfig, GitLabMrReference, OutputFormat},
    gitlab_api::{
        discussion::{
            create_discussion, fetch_discussion_latest_discussion_by_user, fetch_discussions,
            modify_discussion,
        },
        user::current_user,
    },
    input::read_input,
};

pub(crate) fn run(command: DiscussionCommand) -> anyhow::Result<()> {
    match command {
        DiscussionCommand::Create { api, mr, body } => discussion_create(api, mr, &body),
        DiscussionCommand::Update { api, mr, id, body } => discussion_update(api, mr, id, &body),
        DiscussionCommand::List { api, mr, format } => discussion_list(api, mr, format),
        DiscussionCommand::UpdateLatest { api, mr, body } => {
            discussion_update_latest(api, mr, &body)
        }
        DiscussionCommand::PutLatest { api, mr, body } => discussion_put(api, mr, &body),
        DiscussionCommand::Latest { api, mr, format } => latest_discussion(api, mr, format),
    }
}

pub(crate) fn discussion_list(
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

pub(crate) fn discussion_create(
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

pub(crate) fn discussion_update(
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

pub(crate) fn discussion_update_latest(
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

pub(crate) fn discussion_put(
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
    )?;

    if let Some(discussion) = discussion {
        let note = discussion
            .notes
            .first()
            .context("Internal Error, empty discussion?")?;

        modify_discussion(&client, mr.project(), mr.merge_request_id(), note.id, &body)?;
        log::info!("discussion updated");
    } else {
        create_discussion(&client, mr.project(), mr.merge_request_id(), &body)?;
        log::info!("discussion created");
    };

    Ok(())
}

pub(crate) fn latest_discussion(
    api: GitLabApiConfig,
    mr: GitLabMrReference,
    format: OutputFormat,
) -> anyhow::Result<()> {
    let client: Gitlab = Gitlab::new(api.url, api.token.expose_secret())?;

    let current_user = current_user(&client)?;

    let discussion = fetch_discussion_latest_discussion_by_user(
        &client,
        mr.project(),
        mr.merge_request_id(),
        current_user.id,
    )?;

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&discussion)?),
    }
    Ok(())
}
