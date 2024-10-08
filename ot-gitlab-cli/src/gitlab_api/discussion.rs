use gitlab::{
    api::{projects::merge_requests::discussions, Query as _},
    Gitlab,
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
    pub type_: NoteType,
    pub body: String,
    pub author: NoteAuthor,
    // pub created_at
    // pub updated_at
    pub system: bool,
    pub noteable_id: u64,
    pub noteable_type: NoteableType,
    pub project_id: u64,
    pub noteable_iid: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(from = "Option<NoteType>")]
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
    let call = discussions::MergeRequestDiscussions::builder()
        .project(project)
        .merge_request(merge_request)
        .build()?;

    let discussions = call.query(client)?;
    Ok(discussions)
}
