use askama::Template;
use async_trait::async_trait;

use crate::{
    backend::engine::SDKEngine,
    cognition::{
        operations::{SubdivideTaskInput, TaskSuggestion, TaskSuggestionInput},
        suggestions::CognitionCapabilities,
    },
    common::commons::SortOrder,
    errors::sdk::SDKError,
    resources::{
        projects::{operations::ProjectCrudOperations, project::Project},
        tasks::{
            operations::{GetTasksInputBuilder, GetTasksWhereBuilder, TaskCrudOperations},
            task::Task,
        },
    },
};

#[async_trait]
pub trait CognitionOperationsV2 {
    async fn get_suggestions_v2(&self, input: TaskSuggestionInput) -> Result<TaskSuggestion, SDKError>;
    async fn subdivide_task_v2(&self, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>, SDKError>;
}

fn calculate_task_fingerprint(task: &Task) -> String {
    serde_json::to_string_pretty(&task).unwrap()
}

fn calculate_project_fingerprint(project: &Project) -> String {
    serde_json::to_string_pretty(&project).unwrap()
}

fn calculate_task_suggestion_input_fingerprint(input: &TaskSuggestionInput) -> String {
    serde_json::to_string_pretty(&input).unwrap()
}

fn current_time() -> String {
    chrono::Local::now().to_string()
}

#[derive(Template)]
#[template(path = "task_suggestion.md.jinja", ext = "plain")]
pub struct TaskSuggestionTemplate {
    tasks: Vec<Task>,
    initial_state: Option<TaskSuggestionInput>,
    project: Option<Project>,
    user_query: Option<String>,
}

#[derive(Template)]
#[template(path = "task_subdivide.md.jinja", ext = "plain")]
pub struct TaskSubdivideTemplate {
    parent_task: Task,
    number_of_subtasks: u8,
    project: Option<Project>,
    tasks: Option<Vec<Task>>,
    user_query: Option<String>,
}

#[derive(Template)]
#[template(path = "plexo_system.md.jinja", ext = "plain")]
pub struct PlexoSystemTemplate {}

#[async_trait]
impl CognitionOperationsV2 for SDKEngine {
    async fn get_suggestions_v2(&self, input: TaskSuggestionInput) -> Result<TaskSuggestion, SDKError> {
        let system_message = PlexoSystemTemplate {}.render().unwrap();

        let (tasks, project) = match input.project_id {
            Some(project_id) => {
                let project = self.get_project(project_id).await?;

                (
                    self.get_tasks(
                        GetTasksInputBuilder::default()
                            .filter(GetTasksWhereBuilder::default().project_id(project_id).build().unwrap())
                            .sort_by("created_at".to_string())
                            .sort_order(SortOrder::Asc)
                            .limit(10)
                            .build()
                            .ok(),
                    )
                    .await?,
                    Some(project),
                )
            }

            None => (
                self.get_tasks(
                    GetTasksInputBuilder::default()
                        .sort_by("created_at".to_string())
                        .sort_order(SortOrder::Asc)
                        .limit(10)
                        .build()
                        .ok(),
                )
                .await?,
                None,
            ),
        };

        let input_message = TaskSuggestionTemplate {
            tasks,
            project,
            initial_state: Some(input),
            user_query: None,
        }
        .render()
        .unwrap();

        let result = self.chat_completion(system_message, input_message).await;
        let result = result.trim().trim_matches('`');

        let suggestion_result: TaskSuggestion = serde_json::from_str(result).inspect_err(|err| {
            println!("Error parsing suggestion result: {:?}", err);
        })?;

        Ok(suggestion_result)
    }

    async fn subdivide_task_v2(&self, input: SubdivideTaskInput) -> Result<Vec<TaskSuggestion>, SDKError> {
        let system_message = PlexoSystemTemplate {}.render().unwrap();

        let parent_task = self.get_task(input.task_id).await?;

        let (project, project_id) = match parent_task.project_id {
            Some(project_id) => (Some(self.get_project(project_id).await?), Some(project_id)),
            None => (None, None),
        };

        let tasks = match (input.with_tasks_context, project_id) {
            (Some(true), Some(project_id)) => {
                let tasks = self
                    .get_tasks(
                        GetTasksInputBuilder::default()
                            .filter(GetTasksWhereBuilder::default().project_id(project_id).build().unwrap())
                            .sort_by("created_at".to_string())
                            .sort_order(SortOrder::Desc)
                            .limit(10)
                            .build()
                            .ok(),
                    )
                    .await?;

                Some(tasks)
            }
            (Some(true), None) => {
                let tasks = self
                    .get_tasks(
                        GetTasksInputBuilder::default()
                            .sort_by("created_at".to_string())
                            .sort_order(SortOrder::Desc)
                            .limit(10)
                            .build()
                            .ok(),
                    )
                    .await?;

                Some(tasks)
            }
            (None, _) | (Some(false), _) => None,
        };

        let input_message = TaskSubdivideTemplate {
            parent_task,
            number_of_subtasks: input.subtasks,
            project,
            tasks,
            user_query: None,
        }
        .render()
        .unwrap();

        let result = self.chat_completion(system_message, input_message).await;
        let result = result.trim().trim_matches('`');

        let subtasks: Vec<TaskSuggestion> = serde_json::from_str(result).inspect_err(|err| {
            println!("Error parsing subtasks result: {:?}", err);
        })?;

        Ok(subtasks)
    }
}
