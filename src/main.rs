use std::{env::var, error::Error};

use plexo_sdk::{
    backend::engine::new_postgres_engine,
    tasks::{
        operations::{
            CreateTaskInputBuilder, GetTasksByBuilder, GetTasksInput, GetTasksInputBuilder,
            TaskOperations,
        },
        task::{TaskPriority, TaskStatus},
    },
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_url = var("DATABASE_URL").unwrap();

    let engine = new_postgres_engine(database_url.as_str()).await.unwrap();

    let task = engine
        .create_task(
            CreateTaskInputBuilder::default()
                .owner_id(Uuid::new_v4())
                .status(TaskStatus::InProgress)
                .priority(TaskPriority::High)
                .title("Test task".to_string())
                .description("Test description".to_string())
                .build()?,
        )
        .await?;

    println!("{:?}", task);

    let tasks_filter = GetTasksInputBuilder::default()
        .task(
            GetTasksByBuilder::default()
                ._and(vec![
                    GetTasksByBuilder::default()
                        .status(TaskStatus::InProgress)
                        .build()?,
                    GetTasksByBuilder::default()
                        .status(TaskStatus::Done)
                        .build()?,
                ])
                .build()?,
        )
        .limit(10)
        .build()?;

    let tasks = engine.get_tasks(tasks_filter).await?;

    println!("{:?}", tasks);

    Ok(())
}
