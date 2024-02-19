use std::{env::var, error::Error};

use dotenv::dotenv;
use plexo_sdk::{
    backend::engine::new_postgres_engine,
    cognition::operations::{CognitionOperations, SubdivideTaskInputBuilder, TaskSuggestionInputBuilder},
    resources::tasks::{
        operations::{GetTasksInputBuilder, GetTasksWhereBuilder, TaskCrudOperations},
        // relations::TaskRelations,
        task::TaskStatus,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database_url = var("DATABASE_URL").unwrap();
    let llm_api_key = var("OPENAI_API_KEY").unwrap();
    let llm_model_name = var("OPENAI_MODEL_NAME").unwrap_or("gpt-3.5-turbo".to_string());

    let engine = new_postgres_engine(database_url.as_str(), false, llm_api_key, llm_model_name).await?;

    let tasks_filter = GetTasksInputBuilder::default()
        .filter(
            GetTasksWhereBuilder::default()
                ._or(vec![
                    GetTasksWhereBuilder::default().status(TaskStatus::InProgress).build()?,
                    GetTasksWhereBuilder::default().status(TaskStatus::Done).build()?,
                ])
                .build()?,
        )
        .limit(10)
        .build()?;

    let tasks = engine.get_tasks(Some(tasks_filter)).await?;

    println!("total tasks: {}", tasks.len());

    let suggestion = engine
        .get_suggestions(
            TaskSuggestionInputBuilder::default()
                .title("test".to_string())
                .build()?,
        )
        .await?;

    println!("suggestion: {:?}", suggestion);

    let task_id = tasks.first().unwrap().id;

    let subtasks = engine
        .subdivide_task(
            SubdivideTaskInputBuilder::default()
                .subtasks(3)
                .task_id(task_id)
                .build()?,
        )
        .await?;

    println!("subtasks: {:?}", subtasks);

    Ok(())
}
