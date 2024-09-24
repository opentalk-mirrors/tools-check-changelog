mod gitlab;
mod input;

use std::io::{self, Read as _};

use anyhow::{bail, Context, Ok, Result};
use clap::Parser;
use futures::{stream, StreamExt as _, TryStreamExt};
use gitlab::MergeRequest;
use input::{CliffCommit, CliffContext};
use reqwest::{header::HeaderMap, Client, StatusCode};
use secrecy::{ExposeSecret as _, SecretString};
use url::Url;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[arg(long, env = "GITLAB_API_URL")]
    gitlab_api: Url,

    #[arg(long, env = "GITLAB_TOKEN")]
    gitlab_token: SecretString,

    #[arg(long, env = "GITLAB_REPO")]
    gitlab_repo: String,

    #[arg(long, default_value_t = 20)]
    max_concurrent: usize,
}

fn read_from_stdin() -> Result<Vec<u8>> {
    // read task from stdin
    let mut buffer = Vec::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_end(&mut buffer)
        .context("Failed to read stdin")?;
    log::debug!("Read input fromm stdin");

    Ok(buffer)
}

async fn add_mr_to_commit(
    client: Client,
    gitlab_url: &Url,
    gitlab_repo: &str,
    commit: &mut CliffCommit,
) -> Result<()> {
    let url = format!(
        "{}/projects/{}/repository/commits/{}/merge_requests",
        gitlab_url, gitlab_repo, commit.id
    );

    log::debug!("Request url `{}`", url);
    let res = client.get(url).send().await?;
    if res.status() == StatusCode::NOT_FOUND {
        log::warn!("Commit not found {}", commit.id,);
        return Ok(());
    } else if !res.status().is_success() {
        bail!(
            "Error while querying GitLab: {}",
            res.text()
                .await
                .as_deref()
                .unwrap_or("<failed to receive GitLab response>")
        )
    }
    let mut merge_requests: Vec<MergeRequest> = res
        .json()
        .await
        .with_context(|| format!("Failed to process GitLab response for commit {}", commit.id))?;
    log::debug!(
        "Found {} merge requests for commit {}",
        merge_requests.len(),
        commit.id
    );

    // Filter all closed, locked MRs and sort by IID so that we can take the MR with the highest number
    merge_requests.retain(|mr| !mr.state.is_closed() && !mr.state.is_locked());
    merge_requests.sort_by(|a, b| a.iid.cmp(&b.iid));

    // We take the newest/latest merge request
    let Some(relevant_mr) = merge_requests.last() else {
        log::warn!("No merge request found for commit {}", commit.id);
        return Ok(());
    };

    commit.gitlab.pr_number = Some(relevant_mr.iid);
    commit.gitlab.pr_title = Some(relevant_mr.title.clone());

    log::debug!("added merge request info to commit {}", commit.id);
    Ok(())
}

async fn add_merge_request_information(
    gitlab_token: &SecretString,
    gitlab_url: &Url,
    gitlab_repo: &str,
    concurrency: usize,
    context: &mut CliffContext,
) -> Result<()> {
    let commit_iter = context.iter_mut().flat_map(|context| &mut context.commits);

    let mut header = HeaderMap::new();
    header.insert(
        "PRIVATE-TOKEN",
        gitlab_token.expose_secret().parse().unwrap(),
    );

    let client = Client::builder()
        .default_headers(header)
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("Must be able to build HTTP client");

    let url_encoded_repo = urlencoding::encode(gitlab_repo).into_owned();

    let url = format!("{}/projects/{}", gitlab_url, url_encoded_repo);

    log::debug!("Request url `{}`", url);
    let res = client.get(url).send().await?;
    if !res.status().is_success() {
        log::error!(
            "Error response: {}",
            res.text()
                .await
                .as_deref()
                .unwrap_or("<error while reading GitLab response>")
        );
        bail!("Couldn't verify that repo {} exists", gitlab_repo)
    }

    stream::iter(commit_iter.map(|commit| async {
        add_mr_to_commit(client.clone(), gitlab_url, &url_encoded_repo, commit).await
    }))
    .buffer_unordered(concurrency)
    .try_collect()
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    log::info!("Starting {}", env!("CARGO_BIN_NAME"));

    let input = read_from_stdin()?;

    let mut context: CliffContext =
        serde_json::from_slice(&input).context("Failed to parse input")?;
    log::info!("parsed input");

    add_merge_request_information(
        &cli.gitlab_token,
        &cli.gitlab_api,
        &cli.gitlab_repo,
        cli.max_concurrent,
        &mut context,
    )
    .await?;

    print!("{}", serde_json::to_string(&context)?);
    log::info!("adjusted cliff context was printed");

    Ok(())
}
