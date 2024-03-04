use std::{error::Error, sync::Arc};

use dotenv::dotenv;

use plexo_sdk::{
    backend::engine::{SDKConfig, SDKEngine},
    cognition::{
        operations::{SubdivideTaskInputBuilder, TaskSuggestionInputBuilder},
        v2::{operations::CognitionOperationsV2, projects::ProjectSuggestionInputBuilder},
    },
    common::commons::SortOrder,
    resources::{
        projects::operations::{GetProjectsInputBuilder, ProjectCrudOperations},
        tasks::operations::{GetTasksInputBuilder, TaskCrudOperations},
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let engine = SDKEngine::new(SDKConfig::from_env()).await?;
    let engine = Arc::new(engine);

    println!("version: {:?}", engine.version()?);

    let projects = engine.get_projects(GetProjectsInputBuilder::default().build()?).await?;
    let project = projects.first().unwrap().to_owned();

    let task = engine
        .get_tasks(
            GetTasksInputBuilder::default()
                .sort_by("created_at".to_string())
                .sort_order(SortOrder::Asc)
                .limit(1)
                .build()
                .ok(),
        )
        .await?
        .first()
        .unwrap()
        .to_owned();

    // let user_query = "...".to_string();

    let suggested_task = engine
        .get_suggestions_v2(
            TaskSuggestionInputBuilder::default()
                .project_id(project.id)
                .title("next big update".to_string())
                .build()?,
        )
        .await?;

    println!("suggestion: {:?}", suggested_task);

    let subdivided_tasks = engine
        .subdivide_task_v2(
            SubdivideTaskInputBuilder::default()
                .task_id(task.id)
                .subtasks(3)
                .build()?,
        )
        .await?;

    println!("subdivided tasks: {:?}", subdivided_tasks);

    let project_suggestion = engine
        .get_project_suggestion(
            ProjectSuggestionInputBuilder::default()
                .description("A new project based on modern web".to_string())
                .generate_tasks_number(3)
                .build()?,
        )
        .await?;

    println!("project suggestion: {:?}", project_suggestion);

    Ok(())
}
