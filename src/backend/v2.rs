use std::time::Duration;

use async_openai::{config::OpenAIConfig, Client};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{
    auth::engine::AuthEngine,
    errors::sdk::SDKError,
    organization::operations::{
        CreateOrganizationInput, Organization, OrganizationCrudOperations, SetOrganizationInputBuilder,
        GLOBAL_ORGANIZATION_SETTINGS_NAME,
    },
};

use super::{
    config::{SDKConfig, VERSION},
    context::EngineContext,
    loaders::SDKLoaders,
};

pub trait EngineState {}

// #[derive(Clone)]
pub struct WithContext {
    pub context: EngineContext,
    pub loaders: SDKLoaders,
}

// impl WithContext {
//     pub fn new(member_id: Uuid, organization_id: Option<Uuid>) -> WithContext {
//         WithContext {
//             context: EngineContext {
//                 member_id,
//                 organization_id,
//             },
//         }
//     }
// }

impl EngineState for WithContext {}

#[derive(Clone)]
pub struct WithoutContext;
impl EngineState for WithoutContext {}

#[derive(Clone)]
pub struct Engine<State: EngineState> {
    pub state: State,

    pub config: SDKConfig,
    pub db_pool: Box<Pool<Postgres>>,
    pub llm_client: Box<Client<OpenAIConfig>>,
    pub auth: Box<AuthEngine>,
}

// impl Engine<WithoutContext> {
//     pub async fn initialize_organization(
//         &self,
//         ctx: &EngineContext,
//         value: CreateOrganizationInput,
//     ) -> Result<Organization, SDKError> {
//         let org_serialized = serde_json::to_string(&value)?;

//         let org = self
//             .set_organization_setting(
//                 SetOrganizationInputBuilder::default()
//                     .owner_id(ctx.member_id)
//                     .name(GLOBAL_ORGANIZATION_SETTINGS_NAME.to_string())
//                     .value(org_serialized)
//                     .build()
//                     .unwrap(),
//             )
//             .await?;

//         Ok(org.into())
//     }
// }

// impl<AnyState> Engine<AnyState>
// where
//     AnyState: EngineState,
// {
//     pub async fn new_with_context(ctx: &EngineContext, config: SDKConfig) -> Result<Engine<WithContext>, SDKError> {
//         Engine::<WithoutContext>::new_without_context(config)
//             .await?
//             .with_context(ctx)
//             .await
//     }

//     pub async fn new_without_context(config: SDKConfig) -> Result<Engine<WithoutContext>, SDKError> {
//         let pool = PgPoolOptions::new()
//             .max_connections(10)
//             .acquire_timeout(Duration::from_secs(60))
//             .connect(config.database_url.as_str())
//             .await?;

//         let llm_config = OpenAIConfig::default().with_api_key(config.llm_api_key.clone());

//         let llm_client = Box::new(Client::with_config(llm_config));

//         let db_pool = Box::new(pool);

//         let auth = Box::new(AuthEngine::new(
//             config.jwt_access_token_secret.clone(),
//             config.jwt_refresh_token_secret.clone(),
//             config.github_client_id.clone(),
//             config.github_client_secret.clone(),
//             config.github_redirect_url.clone(),
//         ));

//         Ok(Engine {
//             config,
//             db_pool,
//             llm_client,
//             auth,
//             state: WithoutContext {},
//         })
//     }

//     pub async fn migrate(&self) -> Result<(), SDKError> {
//         sqlx::migrate!().run(self.db_pool.as_ref()).await?;
//         Ok(())
//     }

//     pub fn version(&self) -> Result<String, SDKError> {
//         match VERSION {
//             Some(version) => Ok(version.to_string()),
//             None => Err(SDKError::VersionNotFound),
//         }
//     }
// }

// impl Engine<WithoutContext> {
//     pub async fn with_context(self, ctx: &EngineContext) -> Result<Engine<WithContext>, SDKError> {
//         Ok(Engine {
//             config: self.config,
//             db_pool: self.db_pool,
//             llm_client: self.llm_client,
//             auth: self.auth,
//             state: WithContext {
//                 context: ctx.clone(),
//                 loaders: SDKLoaders::new(self.clone()),
//             },
//         })
//     }
// }

// impl Engine<WithContext> {
//     pub fn without_context(&self) -> Engine<WithoutContext> {
//         // Engine {
//         //     _state: PhantomData,
//         //     config: self.config,
//         //     db_pool: self.db_pool,
//         //     llm_client: self.llm_client,
//         //     context: None,
//         // }
//         Engine::<WithoutContext> {
//             config: self.config.clone(),
//             db_pool: self.db_pool.clone(),
//             llm_client: self.llm_client.clone(),
//             auth: self.auth.clone(),
//             state: WithoutContext {},
//         }
//     }
// }

// impl Engine<WithContext> {
//     pub async fn initialize_organization(&self, value: CreateOrganizationInput) -> Result<Organization, SDKError> {
//         let ctx = self.state.context.clone();

//         self.without_context().initialize_organization(&ctx, value).await
//     }
// }
