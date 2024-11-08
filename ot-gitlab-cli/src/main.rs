use clap::Parser as _;
use ot_gitlab_cli::args::AppArgs;

fn main() -> anyhow::Result<()> {
    let app_args = AppArgs::parse();

    env_logger::Builder::new()
        .filter_level(app_args.verbose.log_level_filter())
        .init();

    log::info!("Starting {}", env!("CARGO_BIN_NAME"));

    ot_gitlab_cli::app::run(app_args)
}
