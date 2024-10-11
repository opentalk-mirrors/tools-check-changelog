use crate::args::{AppArgs, GitlabCommand};

mod discussion;
mod project;
mod user;

pub fn run(args: AppArgs) -> anyhow::Result<()> {
    match args.command {
        GitlabCommand::Discussion(discussion_command) => discussion::run(discussion_command),
        GitlabCommand::User(user_command) => user::run(user_command),
        GitlabCommand::Project(project_command) => project::run(project_command),
    }
}
