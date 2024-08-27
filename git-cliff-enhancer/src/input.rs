//! This module contains definitions from git cliff.
//!
//! The structs [`CliffCommit`] and [`CliffRemoteContributor`] are copied and
//! most fields removed. The field `values` was added to catch additional, unknown
//! or unused fields.
//!
//! Source: `https://github.com/orhun/git-cliff/`

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type CliffContext = Vec<CliffVersionContext>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CliffVersionContext {
    pub commits: Vec<CliffCommit>,

    #[serde(flatten)]
    pub values: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CliffCommit {
    pub id: String,
    pub message: String,
    pub gitlab: CliffRemoteContributor,

    #[serde(flatten)]
    pub values: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CliffRemoteContributor {
    pub username: Option<String>,
    pub pr_title: Option<String>,
    pub pr_number: Option<i64>,
    pub pr_labels: Vec<String>,
    pub is_first_time: bool,

    #[serde(flatten)]
    pub values: BTreeMap<String, Value>,
}
