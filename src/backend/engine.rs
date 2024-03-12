use std::{env::var, time::Duration};

use async_openai::{config::OpenAIConfig, Client};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;
// use tokio::runtime::Handle;

use crate::{
    auth::engine::AuthEngine,
    errors::sdk::SDKError,
    organization::operations::{
        CreateOrganizationInput, Organization, OrganizationCrudOperations, SetOrganizationInputBuilder,
        GLOBAL_ORGANIZATION_SETTINGS_NAME,
    }, // resources::tasks::task::Task,
};
// use crossbeam_channel::unbounded;

pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Clone)]
pub struct SDKConfig {
    pub database_url: String,
    pub llm_api_key: String,
    pub llm_model_name: String,
    pub jwt_access_token_secret: String,
    pub jwt_refresh_token_secret: String,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub github_redirect_url: Option<String>,
}

impl SDKConfig {
    pub fn from_env() -> SDKConfig {
        let database_url = var("DATABASE_URL").unwrap();
        let llm_api_key = var("OPENAI_API_KEY").unwrap();
        let llm_model_name = var("OPENAI_MODEL_NAME").unwrap_or("gpt-3.5-turbo".to_string());

        let jwt_access_token_secret = var("JWT_ACCESS_TOKEN_SECRET").unwrap_or("secret".to_string());
        let jwt_refresh_token_secret = var("JWT_REFRESH_TOKEN_SECRET").unwrap_or(jwt_access_token_secret.clone());
        let github_client_id = var("GITHUB_CLIENT_ID").ok();
        let github_client_secret = var("GITHUB_CLIENT_SECRET").ok();
        let github_redirect_url = var("GITHUB_REDIRECT_URL").ok();

        SDKConfig {
            database_url,
            llm_api_key,
            llm_model_name,
            jwt_access_token_secret,
            jwt_refresh_token_secret,
            github_client_id,
            github_client_secret,
            github_redirect_url,
        }
    }
}

#[derive(Clone)]
pub struct SDKEngine {
    pub config: SDKConfig,
    pub db_pool: Box<Pool<Postgres>>,
    pub llm_client: Box<Client<OpenAIConfig>>,
    pub auth: Box<AuthEngine>,
    // pub task_event_send: crossbeam_channel::Sender<Task>,
    // pub task_event_recv: crossbeam_channel::Receiver<Task>,
}

impl SDKEngine {
    pub async fn new(config: SDKConfig) -> Result<SDKEngine, SDKError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(60))
            .connect(config.database_url.as_str())
            .await?;

        let llm_config = OpenAIConfig::default().with_api_key(config.llm_api_key.clone());

        let llm_client = Box::new(Client::with_config(llm_config));

        let db_pool = Box::new(pool);

        // let (task_event_send, task_event_recv) = unbounded::<Task>();

        let auth = Box::new(AuthEngine::new(
            config.jwt_access_token_secret.clone(),
            config.jwt_refresh_token_secret.clone(),
            config.github_client_id.clone(),
            config.github_client_secret.clone(),
            config.github_redirect_url.clone(),
        ));

        let engine = SDKEngine {
            config,
            db_pool,
            llm_client,
            auth,
        };

        Ok(engine)
    }

    pub async fn migrate(&self) -> Result<(), SDKError> {
        sqlx::migrate!().run(self.db_pool.as_ref()).await?;

        Ok(())
    }

    pub fn version(&self) -> Result<String, SDKError> {
        match VERSION {
            Some(version) => Ok(version.to_string()),
            None => Err(SDKError::VersionNotFound),
        }
    }

    pub async fn initialize_organization(
        &self,
        owner_id: Uuid,
        value: CreateOrganizationInput,
    ) -> Result<Organization, SDKError> {
        let org_serialized = serde_json::to_string(&value)?;

        let org = self
            .set_organization_setting(
                SetOrganizationInputBuilder::default()
                    .owner_id(owner_id)
                    .name(GLOBAL_ORGANIZATION_SETTINGS_NAME.to_string())
                    .value(org_serialized)
                    .build()
                    .unwrap(),
            )
            .await?;

        Ok(org.into())
    }
}
