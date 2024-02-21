use std::env::var;

use async_openai::{config::OpenAIConfig, Client};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::errors::sdk::SDKError;

#[derive(Clone)]
pub struct SDKConfig {
    pub database_url: String,
    pub llm_api_key: String,
    pub llm_model_name: String,

    pub admin_default_email: String,
    pub admin_default_password: String,
}

impl SDKConfig {
    pub fn from_env() -> SDKConfig {
        let database_url = var("DATABASE_URL").unwrap();
        let llm_api_key = var("OPENAI_API_KEY").unwrap();
        let llm_model_name = var("OPENAI_MODEL_NAME").unwrap_or("gpt-3.5-turbo".to_string());
        let admin_default_email = var("ADMIN_DEFAULT_EMAIL").unwrap_or("admin@plexo.app".to_string());
        let admin_default_password = var("ADMIN_DEFAULT_PASSWORD").unwrap_or("admin".to_string());

        SDKConfig {
            database_url,
            llm_api_key,
            llm_model_name,
            admin_default_email,
            admin_default_password,
        }
    }
}

#[derive(Clone)]
pub struct SDKEngine {
    pub config: SDKConfig,
    pub db_pool: Box<Pool<Postgres>>,
    pub llm_client: Box<Client<OpenAIConfig>>,
    // pub llm_model_name: String,
}

impl SDKEngine {
    pub async fn new(config: SDKConfig) -> Result<SDKEngine, SDKError> {
        let pool = PgPoolOptions::new()
            .max_connections(3)
            .connect(config.database_url.as_str())
            .await?;

        let llm_config = OpenAIConfig::default().with_api_key(config.llm_api_key.clone());

        let llm_client = Box::new(Client::with_config(llm_config));

        let db_pool = Box::new(pool);

        Ok(SDKEngine {
            config,
            db_pool,
            llm_client,
            // llm_model_name,
        })
    }

    pub async fn migrate(&self) -> Result<(), SDKError> {
        sqlx::migrate!().run(self.db_pool.as_ref()).await?;

        Ok(())
    }
}
