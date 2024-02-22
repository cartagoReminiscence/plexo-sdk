use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};

use poem_openapi::Object;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use poem_openapi::Enum as OpenApiEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, SimpleObject, Object, Clone, Serialize)]
#[graphql(name = "SDKChange")]
pub struct Change {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub owner_id: Uuid,
    pub resource_id: Uuid,

    pub operation: ChangeOperation,
    pub resource_type: ChangeResourceType,

    pub diff_json: String,
}

#[derive(Debug, Enum, OpenApiEnum, Copy, Clone, Display, EnumString, Deserialize, Serialize, Eq, PartialEq)]
pub enum ChangeOperation {
    Create,
    Update,
    Delete,
}

#[derive(Debug, Enum, OpenApiEnum, Copy, Clone, Display, EnumString, Deserialize, Serialize, Eq, PartialEq)]
pub enum ChangeResourceType {
    Task,
    Project,
    Member,
    Team,
    Asset,
    Label,
    Change,
}
