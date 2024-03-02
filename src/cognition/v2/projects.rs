use async_graphql::{InputObject, SimpleObject};

use derive_builder::Builder;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

use crate::{
    cognition::operations::TaskSuggestion,
    resources::projects::project::{ProjectStatus, ProjectVisibility},
};

#[derive(Default, Builder, Object, InputObject, Serialize)]
#[builder(pattern = "owned")]
pub struct ProjectSuggestionInput {
    pub description: String,

    #[builder(setter(strip_option), default)]
    pub initial_tasks: Option<Vec<TaskSuggestion>>,

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
