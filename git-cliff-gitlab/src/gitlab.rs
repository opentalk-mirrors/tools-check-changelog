use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MergeRequest {
    pub id: i64,
    /// The internal ID of the merge request in the project.
    pub iid: i64,
    pub project_id: i64,
    pub title: String,
    pub state: MergeRequestState,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MergeRequestState {
    Opened,
    Closed,
    Locked,
    Merged,
}

impl MergeRequestState {
    /// Returns `true` if the merge request state is [`Locked`].
    ///
    /// [`Locked`]: MergeRequestState::Locked
    #[must_use]
    pub fn is_locked(&self) -> bool {
        matches!(self, Self::Locked)
    }

    /// Returns `true` if the merge request state is [`Closed`].
    ///
    /// [`Closed`]: MergeRequestState::Closed
    #[must_use]
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }
}
