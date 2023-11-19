use async_trait::async_trait;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use sqlx::{Postgres, Row};
use uuid::Uuid;

use crate::{backend::engine::Engine, common::commons::SortOrder, errors::sdk::SDKError};

use super::project::Project;

#[async_trait]
pub trait ProjectOperations {
    async fn create_project(&self, input: CreateProjectInput) -> Result<Project, SDKError>;
    async fn get_project(&self, id: Uuid) -> Result<Project, SDKError>;
    async fn get_projects(&self, input: GetProjectsInput) -> Result<Vec<Project>, SDKError>;
    async fn update_project(
        &self,
        id: Uuid,
        input: UpdateProjectInput,
    ) -> Result<Project, SDKError>;
    async fn delete_project(&self, id: Uuid) -> Result<Project, SDKError>;
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CreateProjectInput {
    pub name: String,
    pub prefix: Option<String>,
    pub owner_id: Uuid,
    pub description: Option<String>,
    pub lead_id: Option<Uuid>,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct UpdateProjectInput {
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub prefix: Option<String>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct GetProjectsInput {
    pub filter: GetProjectsBy, // Similar to GetTasksBy with relevant fields

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
pub struct GetProjectsBy {
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(strip_option), default)]
    pub prefix: Option<String>,
    #[builder(setter(strip_option), default)]
    pub owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    pub due_date: Option<DateTime<Utc>>,

    #[builder(setter(strip_option), default)]
    pub _and: Option<Vec<GetProjectsBy>>,
    #[builder(setter(strip_option), default)]
    pub _or: Option<Vec<GetProjectsBy>>,
}

impl GetProjectsBy {
    pub fn to_where_clause(&self) -> String {
        let mut where_clause = String::new();
        if let Some(name) = &self.name {
            where_clause.push_str(&format!("name = '{}'", name));
        }
        if let Some(prefix) = &self.prefix {
            where_clause.push_str(&format!("prefix = '{}'", prefix));
        }
        if let Some(owner_id) = &self.owner_id {
            where_clause.push_str(&format!("owner_id = '{}'", owner_id));
        }
        if let Some(description) = &self.description {
            where_clause.push_str(&format!("description = '{}'", description));
        }
        if let Some(lead_id) = &self.lead_id {
            where_clause.push_str(&format!("lead_id = '{}'", lead_id));
        }
        if let Some(start_date) = &self.start_date {
            where_clause.push_str(&format!("start_date = '{}'", start_date));
        }
        if let Some(due_date) = &self.due_date {
            where_clause.push_str(&format!("due_date = '{}'", due_date));
        }
        if let Some(_and) = &self._and {
            where_clause.push_str(&format!(
                "({})",
                _and.iter()
                    .map(|x| x.to_where_clause())
                    .collect::<Vec<String>>()
                    .join(" AND ")
            ));
        }
        if let Some(_or) = &self._or {
            where_clause.push_str(&format!(
                "({})",
                _or.iter()
                    .map(|x| x.to_where_clause())
                    .collect::<Vec<String>>()
                    .join(" OR ")
            ));
        }
        where_clause
    }
}

#[async_trait]
impl ProjectOperations for Engine<Postgres> {
    async fn create_project(&self, input: CreateProjectInput) -> Result<Project, SDKError> {
        let project_final_info = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (name, description, owner_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            input.name,
            input.description,
            input.owner_id,
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(project_final_info)
    }

    async fn get_project(&self, id: Uuid) -> Result<Project, SDKError> {
        let project_info = sqlx::query_as!(
            Project,
            r#"
            SELECT * FROM projects WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(project_info)
    }

    async fn update_project(
        &self,
        id: Uuid,
        input: UpdateProjectInput,
    ) -> Result<Project, SDKError> {
        let project_final_info = sqlx::query_as!(
            Project,
            r#"
            UPDATE projects
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description)
            WHERE id = $3
            RETURNING *
            "#,
            input.name,
            input.description,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(project_final_info)
    }

    async fn delete_project(&self, id: Uuid) -> Result<Project, SDKError> {
        let project_info = sqlx::query_as!(
            Project,
            r#"
            DELETE FROM projects WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(project_info)
    }

    async fn get_projects(&self, input: GetProjectsInput) -> Result<Vec<Project>, SDKError> {
        let where_statement = input.filter.to_where_clause();

        let mut query = format!(
            r#"
            SELECT * FROM projects
            WHERE {}
            "#,
            where_statement
        );

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

        let projects_info = sqlx::query(query.as_str())
            .fetch_all(self.pool.as_ref())
            .await?;

        let projects = projects_info
            .iter()
            .map(|x| Project {
                id: x.get("id"),
                created_at: x.get("created_at"),
                updated_at: x.get("updated_at"),
                name: x.get("name"),
                prefix: x.get("prefix"),
                owner_id: x.get("owner_id"),
                description: x.get("description"),
                lead_id: x.get("lead_id"),
                start_date: x.get("start_date"),
                due_date: x.get("due_date"),
            })
            .collect::<Vec<Project>>();

        Ok(projects)
    }
}
