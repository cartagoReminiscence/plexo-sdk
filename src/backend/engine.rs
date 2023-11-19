use sqlx::{postgres::PgPoolOptions, Database, Pool, Postgres};

use crate::errors::sdk::SDKError;

// trait Engine {}

pub struct Engine<DB>
where
    DB: Database,
{
    pub pool: Box<Pool<DB>>,
    // pub subscription_manager: SubscriptionManager,
    // pub auto_suggestions_engine: AutoSuggestionsEngine,
}

pub async fn new_postgres_engine(database_url: &str) -> Result<Engine<Postgres>, SDKError> {
    Ok(Engine {
        pool: Box::new(
            PgPoolOptions::new()
                .max_connections(3)
                .connect(database_url)
                .await?,
        ),
    })
}
