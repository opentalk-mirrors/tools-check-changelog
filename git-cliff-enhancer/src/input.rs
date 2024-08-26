use std::collections::BTreeMap;

use git_cliff_core::commit::Commit;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type CliffContext<'a> = Vec<CliffVersionContext<'a>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CliffVersionContext<'a> {
    pub commits: Vec<Commit<'a>>,

    #[serde(flatten)]
    pub values: BTreeMap<String, Value>,
}
