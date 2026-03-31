use std::{
    env,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use git_cliff::args::Opt;
use git_cliff_core::error::Result;
use secrecy::ExposeSecret;

/// Name of the environment varible which sets the default branch of a repository.
const ENV_DEFAULT_BRANCH: &str = "DEFAULT_BRANCH";

/// The fallback branch which is used when the [`DEFAULT_BRANCH`] is not set.
const DEFAULT_BRANCH: &str = "main";

static OPENTALK_CONFIG: &str = include_str!("../../git-cliff-config/opentalk.toml");

fn main() -> Result<()> {
    // Parse the command line arguments
    let mut args = Opt::parse();
    if args.verbose == 1 {
        unsafe { env::set_var("RUST_LOG", "debug") };
    } else if args.verbose > 1 {
        unsafe { env::set_var("RUST_LOG", "trace") };
    } else if env::var_os("RUST_LOG").is_none() {
        unsafe { env::set_var("RUST_LOG", "info") };
    }
    git_cliff::logger::init()?;

    // check if custom config was set, use default opentalk config otherwise.
    let config_arg = ConfigOpt::parse();
    if config_arg.no_custom_config() {
        log::debug!("No config was specified, using default opentalk config.");
        let local_config = ensure_local_opentalk_config()?;
        args.config = local_config;
    }

    // Create the configuration file if init flag is given.
    if let Some(path) = &args.init {
        git_cliff::init_config(path.as_deref(), &args.config)?;
        return Ok(());
    }

    // Generate a changelog.
    let changelog = git_cliff::run_with_changelog_modifier(args.clone(), |changelog| {
        if let (Some(gitlab_token), Ok(gitlab_url)) =
            (&args.gitlab_token, env::var("GITLAB_API_URL"))
        {
            let remote = &changelog.config.remote.gitlab;
            let default_branch =
                env::var(ENV_DEFAULT_BRANCH).unwrap_or_else(|_| DEFAULT_BRANCH.to_string());

            git_cliff_gitlab::add_merge_request_information(
                gitlab_token.expose_secret(),
                &gitlab_url.parse()?,
                &format!("{}/{}", remote.owner, remote.repo),
                &default_branch,
                changelog,
            )
            .map_err(|e| {
                log::error!("{e}");
                git_cliff_core::error::Error::ChangelogError(
                    "Error in custom changelog modifier".to_string(),
                )
            })?;
        }
        Ok(())
    })?;

    // Get output destination.
    let output = args
        .output
        .clone()
        .or(changelog.config.changelog.output.clone());
    let out: Box<dyn io::Write> = match &output {
        Some(path) if path == Path::new("-") => Box::new(io::stdout()),
        Some(path) => Box::new(io::BufWriter::new(File::create(path)?)),
        None => Box::new(io::stdout()),
    };

    // Write the changelog.
    let exit_code = match git_cliff::write_changelog(&args, changelog, out) {
        Ok(_) => 0,
        Err(e) => {
            log::error!("{e}");
            1
        }
    };

    process::exit(exit_code);
}

#[derive(Debug, Parser, Clone)]
// We only parse a subset of the available arguments. We do not want to error when receiving unknown
// arguments.
#[command(ignore_errors(true))]
struct ConfigOpt {
    /// Sets the configuration file.
    #[arg(short, long, env = "GIT_CLIFF_CONFIG")]
    pub config: Option<String>,

    /// Sets the URL for the configuration file.
    #[arg(long, env = "GIT_CLIFF_CONFIG_URL")]
    pub config_url: Option<String>,
}

impl ConfigOpt {
    /// Returns true if no custom config was provided, false otherwise.
    fn no_custom_config(&self) -> bool {
        self.config.is_none() && self.config_url.is_none()
    }
}

/// Ensures that the opentalk config is stored locally
fn ensure_local_opentalk_config() -> Result<PathBuf> {
    let project_dirs = directories::ProjectDirs::from("eu.opentalk", "OpenTalk", "OpenTalk Git Cliff")
        .ok_or_else(|| {
            git_cliff_core::error::Error::ChangelogError(
                "Could not store default config. Either specify a config file or ensure a home directory exists.".to_string(),
            )
        })?;
    let config_dir = project_dirs.config_dir();
    let config_path = config_dir.join("opentalk.toml");
    fs::create_dir_all(config_dir)?;
    if config_path.exists() {
        log::info!("Skipping config file creation, config already exists.");
        let bytes = std::fs::read(&config_path)?;
        let local_hash = sha256::digest(&bytes);
        let included_hash = sha256::digest(OPENTALK_CONFIG.as_bytes());
        if local_hash != included_hash {
            log::warn!(
                "Stored config and build-in config differ. Consider deleting '{}' to use default config.",
                config_path.to_string_lossy()
            );
        }
    } else {
        fs::write(&config_path, OPENTALK_CONFIG)?;
        log::info!(
            "Created local config file at '{}'",
            config_path.to_string_lossy()
        );
    }
    Ok(config_path)
}
