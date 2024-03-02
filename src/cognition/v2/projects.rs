use async_graphql::{InputObject, SimpleObject};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::{
    cognition::operations::TaskSuggestion,
    resources::{
        projects::project::{ProjectStatus, ProjectVisibility},
        tasks::task::{TaskPriority, TaskStatus},
    },
};

#[derive(Default, Clone, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct ProjectTaskSuggestionInput {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: DateTime<Utc>,
}

#[derive(Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct ProjectSuggestionInput {
    pub description: String,

    #[builder(setter(strip_option), default)]
    pub initial_tasks: Option<Vec<ProjectTaskSuggestionInput>>,

    #[builder(setter(strip_option), default)]
    pub title: Option<String>,
    #[builder(setter(strip_option), default)]
    pub generate_tasks_number: Option<u8>,
}

#[derive(Debug, Default, Builder, Object, SimpleObject, Deserialize)]
#[builder(pattern = "owned")]
pub struct ProjectSuggestion {
    pub name: String,
    pub status: ProjectStatus,
    pub visibility: ProjectVisibility,

    pub prefix: String,
    pub description: String,

    pub tasks: Option<Vec<TaskSuggestion>>,
}
