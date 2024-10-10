use crate::args::{AppArgs, GitlabCommand};

mod discussion;

pub fn run(args: AppArgs) -> anyhow::Result<()> {
    match args.command {
        GitlabCommand::Discussion(discussion_command) => discussion::run(discussion_command),
    }
}
