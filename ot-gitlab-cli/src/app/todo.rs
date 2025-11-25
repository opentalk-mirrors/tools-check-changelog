use gitlab::Gitlab;
use secrecy::ExposeSecret as _;
use serde::{Deserialize, Serialize};

use crate::{
    args::{GitLabApiConfig, OutputFormat, TodoCommand},
    gitlab_api,
};

pub(crate) fn run(command: TodoCommand) -> anyhow::Result<()> {
    match command {
        TodoCommand::MarkClosedDone { api, format } => mark_closed_done(api, format)?,
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MarkResult {
    marked_as_done: Vec<TodoInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoInfo {
    id: u64,
    target_type: String,
    target_id: u64,
    target_state: String,
    title: String,
}

fn mark_closed_done(api: GitLabApiConfig, format: OutputFormat) -> anyhow::Result<()> {
    let client = Gitlab::new(api.url, api.token.expose_secret())?;

    log::info!("Fetching pending todos...");
    let todos = gitlab_api::todo::get_pending_todos(&client)?;

    let mut marked_as_done = Vec::new();
    let mut skipped = Vec::new();

    for todo in todos {
        let is_closed = matches!(todo.target.state.as_str(), "closed" | "merged");

        let todo_info = TodoInfo {
            id: todo.id,
            target_type: todo.target_type.clone(),
            target_id: todo.target.id,
            target_state: todo.target.state.clone(),
            title: todo.target.title.clone(),
        };

        if is_closed {
            log::info!(
                "Marking todo {} as done: {} #{} ({})",
                todo.id,
                todo.target_type,
                todo.target.iid.unwrap_or(0),
                todo.target.state
            );
            gitlab_api::todo::mark_todo_as_done(&client, todo.id)?;
            marked_as_done.push(todo_info);
        } else {
            log::debug!(
                "Skipping todo {}: {} #{} is still {}",
                todo.id,
                todo.target_type,
                todo.target.iid.unwrap_or(0),
                todo.target.state
            );
            skipped.push(todo_info);
        }
    }

    let result = MarkResult { marked_as_done };

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&result)?),
    }

    Ok(())
}
