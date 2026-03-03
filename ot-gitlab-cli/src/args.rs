use std::{fmt::Display, path::PathBuf, str::FromStr};

use anyhow::{Context, ensure};
use clap::{Args, Parser, Subcommand, ValueEnum};
use secrecy::SecretString;
use url::Url;

#[derive(Debug, Clone, Parser)]
pub struct AppArgs {
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: GitlabCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum GitlabCommand {
    #[command(subcommand)]
    Discussion(DiscussionCommand),

    #[command(subcommand)]
    User(UserCommand),

    #[command(subcommand)]
    Project(ProjectCommand),

    #[command(subcommand)]
    MergeRequest(MergeRequestCommand),

    #[command(subcommand)]
    Todo(TodoCommand),
}

#[derive(Debug, Clone, Subcommand)]
pub enum DiscussionCommand {
    Create {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[command(flatten)]
        mr: GitLabMrReference,

        #[arg(short = 'i', long = "input")]
        body: PathBuf,
    },
    Update {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[command(flatten)]
        mr: GitLabMrReference,

        #[arg(long = "id")]
        id: u64,

        #[arg(short = 'i', long = "input")]
        body: PathBuf,
    },

    /// This will update the latest comment of the current user.
    ///
    /// If no discussion was found for the user, an error is returned.
    UpdateLatest {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[command(flatten)]
        mr: GitLabMrReference,

        #[arg(short = 'i', long = "input")]
        body: PathBuf,
    },

    /// Sets the content of the latest discussion to `body`.
    ///
    /// This will create a new discussion if no discussion for the current user was found.
    PutLatest {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[command(flatten)]
        mr: GitLabMrReference,

        #[arg(short = 'i', long = "input")]
        body: PathBuf,

        /// Resolve the discussion
        #[arg(long = "resolve")]
        resolve: bool,
    },

    /// List all discussions for a merge request.
    List {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[command(flatten)]
        mr: GitLabMrReference,

        #[arg(short = 'f', long = "format")]
        format: OutputFormat,
    },

    /// Print the latest discussion of the user for a merge request.
    Latest {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[command(flatten)]
        mr: GitLabMrReference,

        #[arg(short = 'f', long = "format")]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum UserCommand {
    WhoAmI {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[arg(short = 'f', long = "format")]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum ProjectCommand {
    Get {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[arg(short = 'f', long = "format")]
        format: OutputFormat,

        #[arg(short = 'p', long = "project")]
        project: String,
    },

    AuthorizedCloneUrl {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[arg(short = 'p', long = "project")]
        project: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum MergeRequestCommand {
    GetLabels {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[arg(short = 'f', long = "format")]
        format: OutputFormat,

        #[command(flatten)]
        mr: GitLabMrReference,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum TodoCommand {
    /// Mark all todos as done where the associated merge request or issue is closed
    MarkClosedDone {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[arg(short = 'f', long = "format")]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, Args)]
pub struct GitLabApiConfig {
    /// The domain name of the gitlab instance.
    #[arg(long = "api-url", env = "GITLAB_API_URL", value_parser = parse_url)]
    pub url: String,

    /// The personal access token which is used to authenticate with gitlab.
    #[arg(long = "api-token", env = "GITLAB_TOKEN")]
    pub token: SecretString,
}

/// We only allow a for the domain part. Be more forgiving by trying to parse a
/// url and strip the unwanted parts.
fn parse_url(url: &str) -> anyhow::Result<String> {
    if let Ok(gitlab_url) = Url::from_str(url) {
        ensure!(
            gitlab_url.path() == "/api/v4",
            "Only the domain is allowed, the path is automatically set to `/api/v4`"
        );
        gitlab_url
            .domain()
            .context("Invalid gitlab url")
            .map(String::from)
    } else {
        Ok(url.to_string())
    }
}

#[derive(Debug, Clone, Args)]
pub struct GitLabMrReference {
    /// The merge request reference (e.g. `opentalk/backend/services/roomserver!680`).
    #[arg(short= 'm', long = "merge-request", env = "GITLAB_MR_REF", value_parser = parse_mr_reference)]
    mr_reference: (String, u64),
}

impl GitLabMrReference {
    pub fn project(&self) -> &str {
        self.mr_reference.0.as_str()
    }

    pub fn merge_request_id(&self) -> u64 {
        self.mr_reference.1
    }
}

impl Display for GitLabMrReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}!{}", self.project(), self.merge_request_id())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Json,
}

fn parse_mr_reference(reference: &str) -> anyhow::Result<(String, u64)> {
    let (project, mr_id) = reference
        .split_once('!')
        .context("Expected a merge request id of the form `project/id!123`, missing `!`")?;
    let mr_id: u64 = mr_id.parse().context("Expected a merge request id of the form `project/id!123`, the `!` must be followed by an integer")?;

    Ok((project.to_string(), mr_id))
}
