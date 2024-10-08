use args::AppArgs;
use clap::Parser as _;

mod app;
pub mod args;
pub mod gitlab_api;
pub mod input;

fn main() -> anyhow::Result<()> {
    let app_args = AppArgs::parse();

    env_logger::Builder::new()
        .filter_level(app_args.verbose.log_level_filter())
        .init();

    log::info!("Starting {}", env!("CARGO_BIN_NAME"));

    app::run(app_args)
}
