use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, SimpleObject, Object, Clone)]
pub struct Project {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub prefix: Option<String>,

    pub owner_id: Uuid,
    pub description: Option<String>,

    pub lead_id: Option<Uuid>,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
}
