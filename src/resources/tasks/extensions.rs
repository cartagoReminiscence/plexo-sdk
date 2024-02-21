use std::str::FromStr;

use async_graphql::InputObject;
use async_trait::async_trait;
use derive_builder::Builder;
use sqlx::Row;

use crate::{backend::engine::SDKEngine, errors::sdk::SDKError};

use super::{
    operations::CreateTaskInput,
    task::{Task, TaskPriority, TaskStatus},
};

#[derive(Default, Builder, InputObject)]
#[builder(pattern = "owned")]
pub struct CreateTasksInput {
    pub tasks: Vec<CreateTaskInput>,
}

#[async_trait]
pub trait TasksExtensionOperations {
    async fn create_tasks(&self, input: CreateTasksInput) -> Result<Vec<Task>, SDKError>;
}

#[async_trait]
impl TasksExtensionOperations for SDKEngine {
    async fn create_tasks(&self, input: CreateTasksInput) -> Result<Vec<Task>, SDKError> {
        let values = input
            .tasks
            .iter()
            .map(|task| {
                format!(
                    "('{}', '{}', {}, '{}', '{}', {}, {}, {}, {})",
                    task.title,
                    task.owner_id,
                    task.description
                        .clone()
                        .map(|d| format!("'{}'", d))
                        .unwrap_or("null".to_string()),
                    task.status.unwrap_or_default(),
                    task.priority.unwrap_or_default(),
                    task.due_date
                        .map(|dd| format!("'{}'", dd))
                        .unwrap_or("null".to_string()),
                    task.project_id
                        .map(|p| format!("'{}'", p))
                        .unwrap_or("null".to_string()),
                    task.lead_id.map(|l| format!("'{}'", l)).unwrap_or("null".to_string()),
                    task.parent_id.map(|p| format!("'{}'", p)).unwrap_or("null".to_string()),
                )
            })
            .collect::<Vec<String>>();

        let query = format!(
            "INSERT INTO tasks (title, owner_id, description, status, priority, due_date, project_id, lead_id, parent_id) VALUES {} RETURNING *",
            values.join(", ")
        );

        let tasks = sqlx::query(query.as_str()).fetch_all(self.db_pool.as_ref()).await?;

        Ok(tasks
            .iter()
            .map(|task_info| Task {
                id: task_info.get("id"),
                created_at: task_info.get("created_at"),
                updated_at: task_info.get("updated_at"),
                title: task_info.get("title"),
                description: task_info.get("description"),
                status: task_info
                    .get::<'_, Option<String>, _>("status")
                    .and_then(|a| TaskStatus::from_str(&a).ok())
                    .unwrap_or_default(),
                priority: task_info
                    .get::<'_, Option<String>, _>("priority")
                    .and_then(|a| TaskPriority::from_str(&a).ok())
                    .unwrap_or_default(),
                due_date: task_info.get("due_date"),
                project_id: task_info.get("project_id"),
                lead_id: task_info.get("lead_id"),
                owner_id: task_info.get("owner_id"),
                count: task_info.get("count"),
                parent_id: task_info.get("parent_id"),
            })
            .collect())
    }
}
