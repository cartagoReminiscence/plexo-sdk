use async_trait::async_trait;
use chrono::{DateTime, Utc};

use derive_builder::Builder;
use sqlx::Postgres;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use crate::backend::engine::Engine;
use crate::errors::sdk::SDKError;
use crate::tasks::task::{Task, TaskPriority, TaskStatus};

#[async_trait]
pub trait TaskOperations {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, SDKError>;
    async fn get_task(&self, id: Uuid) -> Result<Task, SDKError>;
    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<Task, SDKError>;
    async fn delete_task(&self, id: Uuid) -> Result<Task, SDKError>;
    async fn get_tasks(&self, input: GetTasksInput) -> Result<Vec<Task>, SDKError>;
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CreateTaskInput {
    owner_id: Uuid,
    status: TaskStatus,
    priority: TaskPriority,
    title: String,
    description: String,
    due_date: DateTime<Utc>,
    project_id: Uuid,
    lead_id: Uuid,
    parent_id: Uuid,
}

#[async_trait]
impl TaskOperations for Engine<Postgres> {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, SDKError> {
        let task_final_info = sqlx::query!(r#"
            INSERT INTO tasks (title, description, owner_id, status, priority, due_date, project_id, lead_id, parent_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            input.title,
            input.description,
            input.owner_id,
            input.status.to_string(),
            input.priority.to_string(),
            input.due_date,
            input.project_id,
            input.lead_id,
            input.parent_id,
        ).fetch_one(self.pool.as_ref()).await?;

        let task = Task {
            id: task_final_info.id,
            created_at: task_final_info.created_at,
            updated_at: task_final_info.updated_at,
            title: task_final_info.title,
            description: task_final_info.description,
            status: TaskStatus::from_optional_str(&task_final_info.status),
            priority: TaskPriority::from_optional_str(&task_final_info.priority),
            due_date: task_final_info.due_date,
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
            parent_id: task_final_info.parent_id,
        };

        Ok(task)
    }

    async fn get_task(&self, id: Uuid) -> Result<Task, SDKError> {
        todo!()
    }

    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<Task, SDKError> {
        todo!()
    }

    async fn delete_task(&self, id: Uuid) -> Result<Task, SDKError> {
        todo!()
    }

    async fn get_tasks(&self, input: GetTasksInput) -> Result<Vec<Task>, SDKError> {
        todo!()
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct UpdateTaskInput {
    #[builder(setter(into, strip_option))]
    status: Option<TaskStatus>,
    #[builder(setter(into, strip_option))]
    priority: Option<TaskPriority>,
    #[builder(setter(into, strip_option))]
    title: Option<String>,
    #[builder(setter(into, strip_option))]
    description: Option<String>,
    #[builder(setter(into, strip_option))]
    due_date: Option<DateTime<Utc>>,
    #[builder(setter(into, strip_option))]
    project_id: Option<Uuid>,
    #[builder(setter(into, strip_option))]
    lead_id: Option<Uuid>,
    #[builder(setter(into, strip_option))]
    parent_id: Option<Uuid>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct GetTasksBy {
    #[builder(setter(into, strip_option))]
    owner_id: Option<Uuid>,
    #[builder(setter(into, strip_option))]
    status: Option<TaskStatus>,
    #[builder(setter(into, strip_option))]
    priority: Option<TaskPriority>,
    #[builder(setter(into, strip_option))]
    title: Option<String>,
    #[builder(setter(into, strip_option))]
    description: Option<String>,
    #[builder(setter(into, strip_option))]
    due_date: Option<DateTime<Utc>>,
    #[builder(setter(into, strip_option))]
    project_id: Option<Uuid>,
    #[builder(setter(into, strip_option))]
    lead_id: Option<Uuid>,
    #[builder(setter(into, strip_option))]
    parent_id: Option<Uuid>,

    #[builder(setter(into, strip_option))]
    _and: Option<Vec<GetTasksBy>>,
    #[builder(setter(into, strip_option))]
    _or: Option<Vec<GetTasksBy>>,
}

pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct GetTasksInput {
    task: GetTasksBy,

    #[builder(setter(into, strip_option))]
    sort_by: Option<String>,
    #[builder(setter(into, strip_option))]
    sort_order: Option<SortOrder>,

    #[builder(setter(into, strip_option))]
    limit: Option<i32>,
    #[builder(setter(into, strip_option))]
    offset: Option<i32>,
}
