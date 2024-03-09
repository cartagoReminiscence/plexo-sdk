use std::error::Error;

use dotenv::dotenv;

use plexo_sdk::{
    backend::{
        context::EngineContext,
        engine::SDKConfig,
        v2::{Engine, WithContext}, // loaders::SDKLoaders,
    },
    organization::operations::CreateOrganizationInputBuilder,
    resources::projects::operations::{GetProjectsInputBuilder, ProjectCrudOperations},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // let engine = SDKEngine::new(SDKConfig::from_env()).await?;
    // let engine = Arc::new(engine);

    // engine.migrate().await?;

    let config = SDKConfig::from_env();
    // let loaders = SDKLoaders::new(engine.clone());
    let ctx = EngineContext::from_credentials("email", "password").await?;

    let engine = Engine::<WithContext>::new_with_context(&ctx, config).await?;

    println!("version: {:?}", engine.version()?);

    let projects = engine.get_projects(GetProjectsInputBuilder::default().build()?).await?;

    println!("projects: {:?}", projects);

    engine
        .initialize_organization(
            CreateOrganizationInputBuilder::default()
                .owner_id(ctx.member_id)
                .photo_url("https://www.google.com".to_string())
                .name("test org".to_string())
                .email("foo@bar.com".to_string())
                .build()?,
        )
        .await?;

    // let lead = project.lead(&loaders).await?;

    // let task = engine
    //     .get_tasks(
    //         GetTasksInputBuilder::default()
    //             .sort_by("created_at".to_string())
    //             .sort_order(SortOrder::Asc)
    //             .limit(1)
    //             .build()
    //             .ok(),
    //     )
    //     .await?
    //     .first()
    //     .unwrap()
    //     .to_owned();

    // let suggested_task = engine
    //     .get_suggestions_v2(
    //         TaskSuggestionInputBuilder::default()
    //             .project_id(project.id)
    //             .title("next big update".to_string())
    //             .build()?,
    //     )
    //     .await?;

    // println!("suggestion: {:?}", suggested_task);

    // let subdivided_tasks = engine
    //     .subdivide_task_v2(
    //         SubdivideTaskInputBuilder::default()
    //             .task_id(task.id)
    //             .subtasks(3)
    //             .build()?,
    //     )
    //     .await?;

    // println!("subdivided tasks: {:?}", subdivided_tasks);

    // let project_suggestion = engine
    //     .get_project_suggestion(
    //         ProjectSuggestionInputBuilder::default()
    //             .description("A new project based on modern web".to_string())
    //             .generate_tasks_number(3)
    //             .build()?,
    //     )
    //     .await?;

    // println!("project suggestion: {:?}", project_suggestion);

    Ok(())
}
