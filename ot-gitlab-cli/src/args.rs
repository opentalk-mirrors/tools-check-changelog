use std::{fmt::Display, path::PathBuf};

use anyhow::Context;
use clap::{Args, Parser, Subcommand, ValueEnum};
use secrecy::SecretString;

#[derive(Debug, Clone, Parser)]
pub struct AppArgs {
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: DiscussionCommand,
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
    UpdateLatest {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[command(flatten)]
        mr: GitLabMrReference,

        #[arg(short = 'i', long = "input")]
        body: PathBuf,
    },
    List {
        #[command(flatten)]
        api: GitLabApiConfig,

        #[command(flatten)]
        mr: GitLabMrReference,

        #[arg()]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, Args)]
pub struct GitLabApiConfig {
    #[arg(long = "api-url", env = "GITLAB_API_URL")]
    pub url: String,

    #[arg(long = "api-token", env = "GITLAB_TOKEN")]
    pub token: SecretString,
}

#[derive(Debug, Clone, Args)]
pub struct GitLabMrReference {
    #[arg(long = "mr-ref", env = "GITLAB_MR_REF", value_parser = parse_mr_reference)]
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
