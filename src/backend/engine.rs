use std::time::Duration;

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

use super::{
    config::{SDKConfig, VERSION},
    v2::{Engine, WithoutContext},
};
// use crossbeam_channel::unbounded;

impl Engine<WithoutContext> {
    pub async fn new(config: SDKConfig) -> Result<Engine<WithoutContext>, SDKError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(60))
            .connect(config.database_url.as_str())
            .await?;

        let llm_config = OpenAIConfig::default().with_api_key(config.llm_api_key.clone());

        let llm_client = Box::new(Client::with_config(llm_config));

        let db_pool = Box::new(pool);

        let auth = Box::new(AuthEngine::new(
            config.jwt_access_token_secret.clone(),
            config.jwt_refresh_token_secret.clone(),
            config.github_client_id.clone(),
            config.github_client_secret.clone(),
            config.github_redirect_url.clone(),
        ));

        let engine = Engine {
            db_pool,
            config,
            llm_client,
            auth,
            state: WithoutContext {},
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
