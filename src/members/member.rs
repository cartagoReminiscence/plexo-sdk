use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Member {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub email: String,

    pub role: MemberRole,

    pub github_id: Option<String>,
    pub google_id: Option<String>,

    pub photo_url: Option<String>,

    // #[graphql(skip)]
    pub password_hash: Option<String>,
}

#[derive(Debug)]
pub enum MemberRole {
    Admin,
    Member,
    ReadOnly,
}
