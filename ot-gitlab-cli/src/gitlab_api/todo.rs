use anyhow::Context;
use derive_builder::Builder;
use gitlab::{
    api::{endpoint_prelude::*, paged, Client, Pagination, Query as _},
    RestError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
    pub id: u64,
    pub action_name: String,
    pub target_type: String,
    pub target: Target,
    pub target_url: String,
    pub body: String,
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Target {
    pub id: u64,
    pub iid: Option<u64>,
    pub state: String,
    pub title: String,
}

#[derive(Debug, Clone, Builder)]
struct GetTodos {
    #[builder(default, setter(strip_option))]
    state: Option<String>,
}

impl Endpoint for GetTodos {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "todos".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        if let Some(ref state) = self.state {
            params.push("state", state);
        }
        params
    }
}

#[derive(Debug, Clone, Builder)]
struct MarkTodoAsDone {
    id: u64,
}

impl Endpoint for MarkTodoAsDone {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("todos/{}/mark_as_done", self.id).into()
    }
}

impl Pageable for GetTodos {}

pub fn get_pending_todos<C: Client<Error = RestError>>(client: &C) -> anyhow::Result<Vec<Todo>> {
    let endpoint = GetTodosBuilder::default()
        .state("pending".to_string())
        .build()?;

    let paged_endpoint = paged(endpoint, Pagination::AllPerPageLimit(10));

    let mut all_todos = Vec::new();
    for page in paged_endpoint.into_iter(client) {
        let response: serde_json::Value = page.context("Failed to query todos")?;

        all_todos.extend(serde_json::from_value(response));
        log::trace!("todos: {}", all_todos.len())
    }

    let todos = all_todos
        .into_iter()
        .map(serde_json::from_value::<Todo>)
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to decode todos")?;

    log::debug!("Fetched {} pending todos", todos.len());

    Ok(todos)
}

pub fn mark_todo_as_done<C: Client<Error = RestError>>(
    client: &C,
    todo_id: u64,
) -> anyhow::Result<()> {
    let endpoint = MarkTodoAsDoneBuilder::default().id(todo_id).build()?;

    let _response: serde_json::Value = endpoint.query(client)?;
    log::debug!("Marked todo {} as done", todo_id);

    Ok(())
}
