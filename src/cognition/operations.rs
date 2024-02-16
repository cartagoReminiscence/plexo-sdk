use async_graphql::InputObject;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use poem_openapi::Object;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::sdk::SDKError,
    tasks::task::{TaskPriority, TaskStatus},
};

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct TaskSuggestionInput {
    pub project_id: Option<Uuid>,

    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Default, Builder, Object, InputObject, Deserialize)]
#[builder(pattern = "owned")]
pub struct TaskSuggestionResult {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: DateTime<Utc>,
}

#[derive(Default, Builder, Object, InputObject)]
#[builder(pattern = "owned")]
pub struct SubdivideTaskInput {
    pub task_id: Uuid,
    pub subtasks: u8, // TODO: validate it or die
}

#[async_trait]
pub trait CognitionOperations {
    async fn get_suggestions(&self, input: TaskSuggestionInput) -> Result<TaskSuggestionResult, SDKError>;
    async fn subdivide_task(&self, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestionResult>, SDKError>;
}
