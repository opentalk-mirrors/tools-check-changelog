mod gitlab;

use anyhow::{Context as _, Result, bail};
use git_cliff_core::{changelog::Changelog, commit::Commit};
use gitlab::MergeRequest;
use rayon::iter::{IntoParallelRefMutIterator as _, ParallelIterator as _};
use reqwest::{StatusCode, blocking::Client, header::HeaderMap};
use url::Url;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn add_mr_to_commit(
    client: Client,
    gitlab_url: &Url,
    gitlab_repo: &str,
    commit: &mut Commit<'_>,
) -> Result<()> {
    let url = format!(
        "{}/projects/{}/repository/commits/{}/merge_requests",
        gitlab_url, gitlab_repo, commit.id
    );

    log::debug!("Request url `{}`", url);
    let res = client.get(url).send()?;
    if res.status() == StatusCode::NOT_FOUND {
        log::warn!("Commit not found {}", commit.id,);
        return Ok(());
    }
    if !res.status().is_success() {
        bail!(
            "Error while querying GitLab: {}",
            res.text()
                .as_deref()
                .unwrap_or("<failed to receive GitLab response>")
        )
    }
    let mut merge_requests: Vec<MergeRequest> = res
        .json()
        .with_context(|| format!("Failed to process GitLab response for commit {}", commit.id))?;
    log::debug!(
        "Found {} merge requests for commit {}",
        merge_requests.len(),
        commit.id
    );

    // Filter all closed, locked MRs and sort by IID so that we can take the MR with
    // the highest number
    merge_requests.retain(|mr| !mr.state.is_closed() && !mr.state.is_locked());
    merge_requests.sort_by(|a, b| a.iid.cmp(&b.iid));

    // We take the newest/latest merge request
    let Some(relevant_mr) = merge_requests.last() else {
        log::warn!("No merge request found for commit {}", commit.id);
        return Ok(());
    };

    let remote = commit.remote.get_or_insert_default();
    remote.pr_number = Some(relevant_mr.iid);
    remote.pr_title = Some(relevant_mr.title.clone());

    log::debug!("added merge request info to commit {}", commit.id);
    Ok(())
}

pub fn add_merge_request_information(
    gitlab_token: &str,
    gitlab_url: &Url,
    gitlab_repo: &str,
    changelog: &mut Changelog<'_>,
) -> Result<()> {
    let commit_iter = changelog
        .releases
        .par_iter_mut()
        .flat_map(|release| &mut release.commits);

    let mut header = HeaderMap::new();
    header.insert(
        "PRIVATE-TOKEN",
        gitlab_token.parse().context("Invalid GITLAB_TOKEN")?,
    );

    let client = Client::builder()
        .default_headers(header)
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("Must be able to build HTTP client");

    let url_encoded_repo = urlencoding::encode(gitlab_repo).into_owned();

    let url = format!("{}/projects/{}", gitlab_url, url_encoded_repo);

    log::debug!("Request url `{}`", url);
    let res = client.get(url).send()?;
    if !res.status().is_success() {
        log::error!(
            "Error response: {}",
            res.text()
                .as_deref()
                .unwrap_or("<error while reading GitLab response>")
        );
        bail!("Couldn't verify that repo {gitlab_repo} exists");
    }

    commit_iter.try_for_each(|commit| {
        add_mr_to_commit(client.clone(), gitlab_url, &url_encoded_repo, commit)
    })?;

    Ok(())
}
