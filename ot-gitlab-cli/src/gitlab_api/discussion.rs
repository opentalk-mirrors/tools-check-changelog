use anyhow::Context;
use chrono::{DateTime, Utc};
use gitlab::{
    api::{
        paged,
        projects::merge_requests::{discussions, notes},
        Client, Pagination, Query,
    },
    Gitlab, RestError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Discussion {
    pub id: String,
    pub individual_note: bool,
    #[serde(default)]
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    pub id: u64,
    #[serde(rename = "type", default)]
    pub type_: Option<NoteType>,
    pub body: String,
    pub author: NoteAuthor,
    pub created_at: DateTime<Utc>,
    // pub updated_at
    pub system: bool,
    pub noteable_id: u64,
    pub noteable_type: NoteableType,
    // pub project_id: u64,
    // pub noteable_iid: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub enum NoteType {
    #[default]
    Null,
    DiscussionNote,
    DiffNote,
}

impl From<Option<Self>> for NoteType {
    fn from(value: Option<Self>) -> Self {
        value.unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NoteableType {
    Issue,
    MergeRequest,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteAuthor {
    pub id: u64,
    pub name: String,
    pub username: String,
    // state
    // avatar_url
    // web_url
}

pub fn fetch_discussions(
    client: &Gitlab,
    project: &str,
    merge_request: u64,
) -> anyhow::Result<Vec<Discussion>> {
    log::info!("fetch discussions");
    let call = discussions::MergeRequestDiscussions::builder()
        .project(project)
        .merge_request(merge_request)
        .build()?;
    let call = paged(call, Pagination::Limit(100));

    let mut discussions = Vec::new();
    for page in call.into_iter(client) {
        let response: serde_json::Value = page.context("Failed to query discussion")?;
        log::trace!("response: {:#?}", discussions);

        discussions.extend(serde_json::from_value(response));
    }
    let discussions = discussions
        .into_iter()
        .map(serde_json::from_value::<Discussion>)
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to decode discussion")?;

    log::debug!("fetched discussions");
    Ok(discussions)
}

pub fn fetch_latest_discussion_by_user(
    client: &Gitlab,
    project: &str,
    merge_request: u64,
    user: u64,
) -> anyhow::Result<Option<Discussion>> {
    let discussions = fetch_discussions(client, project, merge_request)?;

    // filter by user and sort by created_at of first note in discussion
    let mut discussions_by_user: Vec<_> = discussions
        .into_iter()
        .filter(|d| {
            d.notes
                .first()
                .is_some_and(|n| n.author.id == user && !n.system)
        })
        .collect();
    discussions_by_user.sort_by(|a, b| {
        a.notes
            .first()
            .map(|n| n.created_at)
            .cmp(&b.notes.first().map(|n| n.created_at))
    });

    Ok(discussions_by_user.pop())
}

pub fn create_discussion<C: Client<Error = RestError>>(
    client: &C,
    project: &str,
    merge_request: u64,
    body: &str,
) -> anyhow::Result<Discussion> {
    let call = discussions::CreateMergeRequestDiscussion::builder()
        .project(project)
        .merge_request(merge_request)
        .body(body)
        .build()?;
    let response: serde_json::Value = call.query(client)?;
    log::trace!("response: {:#?}", response);
    let discussion = serde_json::from_value::<Discussion>(response)?;
    Ok(discussion)
}

pub fn modify_discussion<C: Client<Error = RestError>>(
    client: &C,
    project: &str,
    merge_request: u64,
    note: u64,
    body: &str,
) -> anyhow::Result<()> {
    let call = notes::EditMergeRequestNote::builder()
        .project(project)
        .merge_request(merge_request)
        .note(note)
        .body(body)
        .build()?;

    let response: serde_json::Value = call.query(client)?;
    log::trace!("response: {:#?}", response);

    Ok(())
}
