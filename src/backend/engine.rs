use async_openai::{config::OpenAIConfig, Client};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::errors::sdk::SDKError;

#[derive(Clone)]
pub struct SDKEngine {
    pub db_pool: Box<Pool<Postgres>>,
    pub llm_client: Box<Client<OpenAIConfig>>,
    pub llm_model_name: String,
}

pub async fn new_postgres_engine(
    database_url: &str,
    with_migration: bool,
    llm_api_key: String,
    llm_model_name: String,
) -> Result<SDKEngine, SDKError> {
    let pool = PgPoolOptions::new().max_connections(3).connect(database_url).await?;

    if with_migration {
        sqlx::migrate!().run(&pool).await?;
    }

    let config = OpenAIConfig::default().with_api_key(llm_api_key);

    let llm_client = Box::new(Client::with_config(config));

    let db_pool = Box::new(pool);

    Ok(SDKEngine {
        db_pool,
        llm_client,
        llm_model_name,
    })
}
