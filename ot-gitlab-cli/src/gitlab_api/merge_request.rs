use gitlab::{
    api::{projects::merge_requests, Client, Query as _},
    RestError,
};

use crate::args::GitLabMrReference;

pub fn get_label<C: Client<Error = RestError>>(
    client: &C,
    mr: GitLabMrReference,
) -> anyhow::Result<Vec<String>> {
    let call = merge_requests::MergeRequest::builder()
        .project(mr.project())
        .merge_request(mr.merge_request_id())
        .build()?;

    let response: serde_json::Value = call.query(client)?;
    log::trace!("response: {response:#?}");

    // Extract labels from the response
    let labels = response
        .get("labels")
        .and_then(|labels| labels.as_array())
        .map(|labels| {
            labels
                .iter()
                .filter_map(|label| label.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok(labels)
}
