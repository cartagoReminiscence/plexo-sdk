use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};

use poem_openapi::Object;

use uuid::Uuid;

#[derive(Debug, SimpleObject, Object, Clone)]
pub struct Label {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}
