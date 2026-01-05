use crate::args::{AppArgs, GitlabCommand};

mod discussion;
mod merge_request;
mod project;
mod todo;
mod user;

pub fn run(args: AppArgs) -> anyhow::Result<()> {
    match args.command {
        GitlabCommand::Discussion(discussion_command) => discussion::run(discussion_command),
        GitlabCommand::User(user_command) => user::run(user_command),
        GitlabCommand::Project(project_command) => project::run(project_command),
        GitlabCommand::MergeRequest(merge_request_command) => {
            merge_request::run(merge_request_command)
        }
        GitlabCommand::Todo(todo_command) => todo::run(todo_command),
    }
}
