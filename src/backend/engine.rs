use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::errors::sdk::SDKError;

pub struct SDKEngine {
    pub pool: Box<Pool<Postgres>>,
    // pub subscription_manager: SubscriptionManager,
    // pub auto_suggestions_engine: AutoSuggestionsEngine,
}

pub async fn new_postgres_engine(database_url: &str) -> Result<SDKEngine, SDKError> {
    Ok(SDKEngine {
        pool: Box::new(
            PgPoolOptions::new()
                .max_connections(3)
                .connect(database_url)
                .await?,
        ),
    })
}
