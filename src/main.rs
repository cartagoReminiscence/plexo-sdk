use std::{env::var, error::Error};

use dotenv::dotenv;
use plexo_sdk::{
    backend::engine::new_postgres_engine,
    tasks::{
        operations::{GetTasksInputBuilder, GetTasksWhereBuilder, TaskOperations},
        relations::TaskRelations,
        task::TaskStatus,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database_url = var("DATABASE_URL").unwrap();

    let engine = new_postgres_engine(database_url.as_str()).await.unwrap();

    let tasks_filter = GetTasksInputBuilder::default()
        .filter(
            GetTasksWhereBuilder::default()
                ._or(vec![
                    GetTasksWhereBuilder::default()
                        .status(TaskStatus::InProgress)
                        .build()?,
                    GetTasksWhereBuilder::default()
                        .status(TaskStatus::Done)
                        .build()?,
                ])
                .build()?,
        )
        .limit(10)
        .build()?;

    let tasks = engine.get_tasks(tasks_filter).await?;

    println!("total tasks: {}", tasks.len());

    let project = tasks.first().unwrap().project().await?;

    println!("project: {}", project.name);

    Ok(())
}
