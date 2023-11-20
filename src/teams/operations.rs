use std::str::FromStr;

use async_trait::async_trait;

use derive_builder::Builder;
use sqlx::{Row};
use uuid::Uuid;

use crate::{backend::engine::SDKEngine, common::commons::SortOrder, errors::sdk::SDKError};

use super::team::{Team, TeamVisibility};

#[async_trait]
pub trait TeamCrudOperations {
    async fn create_team(&self, input: CreateTeamInput) -> Result<Team, SDKError>;
    async fn get_team(&self, id: Uuid) -> Result<Team, SDKError>;
    async fn get_teams(&self, input: GetTeamsInput) -> Result<Vec<Team>, SDKError>;
    async fn update_team(&self, id: Uuid, input: UpdateTeamInput) -> Result<Team, SDKError>;
    async fn delete_team(&self, id: Uuid) -> Result<Team, SDKError>;
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CreateTeamInput {
    pub name: String,
    pub owner_id: Uuid,
    pub visibility: TeamVisibility,
    pub prefix: Option<String>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct UpdateTeamInput {
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub visibility: Option<TeamVisibility>,
    #[builder(setter(strip_option), default)]
    pub prefix: Option<String>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct GetTeamsInput {
    #[builder(setter(strip_option), default)]
    pub filter: Option<GetTeamsWhere>,

    #[builder(setter(strip_option), default)]
    pub sort_by: Option<String>,
    #[builder(setter(strip_option), default)]
    pub sort_order: Option<SortOrder>,

    #[builder(setter(into, strip_option), default = "Some(100)")]
    pub limit: Option<i32>,
    #[builder(setter(into, strip_option), default = "Some(0)")]
    pub offset: Option<i32>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct GetTeamsWhere {
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub visibility: Option<TeamVisibility>,
    #[builder(setter(strip_option), default)]
    pub prefix: Option<String>,

    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetTeamsWhere>>,
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetTeamsWhere>>,
}

impl GetTeamsWhere {
    pub fn compile_sql(&self) -> String {
        let mut query = String::new();

        if let Some(name) = &self.name {
            query.push_str(format!("name = '{}' ", name).as_str());
        }

        if let Some(owner_id) = &self.owner_id {
            query.push_str(format!("owner_id = '{}' ", owner_id).as_str());
        }

        if let Some(visibility) = &self.visibility {
            query.push_str(format!("visibility = '{}' ", visibility).as_str());
        }

        if let Some(prefix) = &self.prefix {
            query.push_str(format!("prefix = '{}' ", prefix).as_str());
        }

        if let Some(_and) = &self._and {
            query.push_str("AND (");

            for (index, where_clause) in _and.iter().enumerate() {
                query.push_str(where_clause.compile_sql().as_str());

                if index != _and.len() - 1 {
                    query.push_str("AND ");
                }
            }

            query.push_str(") ");
        }

        if let Some(_or) = &self._or {
            query.push_str("OR (");

            for (index, where_clause) in _or.iter().enumerate() {
                query.push_str(where_clause.compile_sql().as_str());

                if index != _or.len() - 1 {
                    query.push_str("OR ");
                }
            }

            query.push_str(") ");
        }

        query
    }
}

#[async_trait]
impl TeamCrudOperations for SDKEngine {
    async fn create_team(&self, input: CreateTeamInput) -> Result<Team, SDKError> {
        let team_final_info = sqlx::query!(
            r#"
            INSERT INTO teams (name, owner_id, visibility, prefix)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            input.name,
            input.owner_id,
            input.visibility.to_string(),
            input.prefix
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        let team = Team {
            id: team_final_info.id,
            created_at: team_final_info.created_at,
            updated_at: team_final_info.updated_at,
            name: team_final_info.name,
            owner_id: team_final_info.owner_id,
            visibility: team_final_info
                .visibility
                .and_then(|a| TeamVisibility::from_str(&a).ok())
                .unwrap_or_default(),
            prefix: team_final_info.prefix,
        };

        Ok(team)
    }

    async fn get_team(&self, id: Uuid) -> Result<Team, SDKError> {
        let team_info = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, name, owner_id, visibility, prefix
            FROM teams
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        let team = Team {
            id: team_info.id,
            created_at: team_info.created_at,
            updated_at: team_info.updated_at,
            name: team_info.name,
            owner_id: team_info.owner_id,
            visibility: team_info
                .visibility
                .and_then(|a| TeamVisibility::from_str(&a).ok())
                .unwrap_or_default(),
            prefix: team_info.prefix,
        };

        Ok(team)
    }

    async fn get_teams(&self, input: GetTeamsInput) -> Result<Vec<Team>, SDKError> {
        let mut query = "SELECT * FROM teams ".to_string();

        if let Some(filter) = input.filter {
            query.push_str(format!("WHERE {} ", filter.compile_sql()).as_str());
        }

        if let Some(sort_by) = input.sort_by {
            query.push_str(format!("ORDER BY {} ", sort_by).as_str());
        }

        if let Some(sort_order) = input.sort_order {
            query.push_str(format!("{} ", sort_order).as_str());
        }

        if let Some(limit) = input.limit {
            query.push_str(format!("LIMIT {} ", limit).as_str());
        }

        if let Some(offset) = input.offset {
            query.push_str(format!("OFFSET {} ", offset).as_str());
        }

        let teams_info = sqlx::query(query.as_str())
            .fetch_all(self.pool.as_ref())
            .await?;

        let teams = teams_info
            .iter()
            .map(|x| Team {
                id: x.get("id"),
                created_at: x.get("created_at"),
                updated_at: x.get("updated_at"),
                name: x.get("name"),
                owner_id: x.get("owner_id"),
                visibility: x
                    .get::<'_, Option<String>, _>("visibility")
                    .and_then(|a| TeamVisibility::from_str(&a).ok())
                    .unwrap_or_default(),
                prefix: x.get("prefix"),
            })
            .collect::<Vec<Team>>();

        Ok(teams)
    }

    async fn update_team(&self, id: Uuid, input: UpdateTeamInput) -> Result<Team, SDKError> {
        let team_final_info = sqlx::query!(
            r#"
            UPDATE teams
            SET
                name = COALESCE($1, name),
                owner_id = COALESCE($2, owner_id),
                visibility = COALESCE($3, visibility),
                prefix = COALESCE($4, prefix),
                updated_at = now()
            WHERE id = $5
            RETURNING *
            "#,
            input.name,
            input.owner_id,
            input.visibility.map(|visibility| visibility.to_string()),
            input.prefix,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        let team = Team {
            id: team_final_info.id,
            created_at: team_final_info.created_at,
            updated_at: team_final_info.updated_at,
            name: team_final_info.name,
            owner_id: team_final_info.owner_id,
            visibility: team_final_info
                .visibility
                .and_then(|a| TeamVisibility::from_str(&a).ok())
                .unwrap_or_default(),
            prefix: team_final_info.prefix,
        };

        Ok(team)
    }

    async fn delete_team(&self, id: Uuid) -> Result<Team, SDKError> {
        let team_info = sqlx::query!(
            r#"
            DELETE FROM teams
            WHERE id = $1
            RETURNING *
            "#,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        let team = Team {
            id: team_info.id,
            created_at: team_info.created_at,
            updated_at: team_info.updated_at,
            name: team_info.name,
            owner_id: team_info.owner_id,
            visibility: team_info
                .visibility
                .and_then(|a| TeamVisibility::from_str(&a).ok())
                .unwrap_or_default(),
            prefix: team_info.prefix,
        };

        Ok(team)
    }
}
