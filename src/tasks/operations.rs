use async_trait::async_trait;
use chrono::{DateTime, Utc};

use derive_builder::Builder;
use sqlx::{Postgres, Row};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use crate::backend::engine::Engine;
use crate::errors::sdk::SDKError;
use crate::tasks::task::{Task, TaskPriority, TaskStatus};

#[async_trait]
pub trait TaskOperations {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, SDKError>;
    async fn get_task(&self, id: Uuid) -> Result<Task, SDKError>;
    async fn get_tasks(&self, input: GetTasksInput) -> Result<Vec<Task>, SDKError>;
    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<Task, SDKError>;
    async fn delete_task(&self, id: Uuid) -> Result<Task, SDKError>;
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CreateTaskInput {
    owner_id: Uuid,
    status: TaskStatus,
    priority: TaskPriority,
    title: String,
    description: String,
    due_date: DateTime<Utc>,
    project_id: Uuid,
    lead_id: Uuid,
    parent_id: Uuid,
}

#[async_trait]
impl TaskOperations for Engine<Postgres> {
    async fn create_task(&self, input: CreateTaskInput) -> Result<Task, SDKError> {
        let task_final_info = sqlx::query!(r#"
            INSERT INTO tasks (title, description, owner_id, status, priority, due_date, project_id, lead_id, parent_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            input.title,
            input.description,
            input.owner_id,
            input.status.to_string(),
            input.priority.to_string(),
            input.due_date,
            input.project_id,
            input.lead_id,
            input.parent_id,
        ).fetch_one(self.pool.as_ref()).await?;

        let task = Task {
            id: task_final_info.id,
            created_at: task_final_info.created_at,
            updated_at: task_final_info.updated_at,
            title: task_final_info.title,
            description: task_final_info.description,
            status: TaskStatus::from_optional_str(&task_final_info.status),
            priority: TaskPriority::from_optional_str(&task_final_info.priority),
            due_date: task_final_info.due_date,
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
            parent_id: task_final_info.parent_id,
        };

        Ok(task)
    }

    async fn get_task(&self, id: Uuid) -> Result<Task, SDKError> {
        let task_info = sqlx::query!(
            r#"
            SELECT * FROM tasks WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        let task = Task {
            id: task_info.id,
            created_at: task_info.created_at,
            updated_at: task_info.updated_at,
            title: task_info.title,
            description: task_info.description,
            status: TaskStatus::from_optional_str(&task_info.status),
            priority: TaskPriority::from_optional_str(&task_info.priority),
            due_date: task_info.due_date,
            project_id: task_info.project_id,
            lead_id: task_info.lead_id,
            owner_id: task_info.owner_id,
            count: task_info.count,
            parent_id: task_info.parent_id,
        };

        Ok(task)
    }

    async fn update_task(&self, id: Uuid, input: UpdateTaskInput) -> Result<Task, SDKError> {
        let task_final_info = sqlx::query!(
            r#"
            UPDATE tasks
            SET
                status = COALESCE($1, status),
                priority = COALESCE($2, priority),
                title = COALESCE($3, title),
                description = COALESCE($4, description),
                due_date = COALESCE($5, due_date),
                project_id = COALESCE($6, project_id),
                lead_id = COALESCE($7, lead_id),
                parent_id = COALESCE($8, parent_id)
            WHERE id = $9
            RETURNING *
            "#,
            input.status.map(|status| status.to_string()),
            input.priority.map(|priority| priority.to_string()),
            input.title,
            input.description,
            input.due_date,
            input.project_id,
            input.lead_id,
            input.parent_id,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        let task = Task {
            id: task_final_info.id,
            created_at: task_final_info.created_at,
            updated_at: task_final_info.updated_at,
            title: task_final_info.title,
            description: task_final_info.description,
            status: TaskStatus::from_optional_str(&task_final_info.status),
            priority: TaskPriority::from_optional_str(&task_final_info.priority),
            due_date: task_final_info.due_date,
            project_id: task_final_info.project_id,
            lead_id: task_final_info.lead_id,
            owner_id: task_final_info.owner_id,
            count: task_final_info.count,
            parent_id: task_final_info.parent_id,
        };

        Ok(task)
    }

    async fn delete_task(&self, id: Uuid) -> Result<Task, SDKError> {
        let task_info = sqlx::query!(
            r#"
            DELETE FROM tasks WHERE id = $1
            RETURNING *
            "#,
            id,
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        let task = Task {
            id: task_info.id,
            created_at: task_info.created_at,
            updated_at: task_info.updated_at,
            title: task_info.title,
            description: task_info.description,
            status: TaskStatus::from_optional_str(&task_info.status),
            priority: TaskPriority::from_optional_str(&task_info.priority),
            due_date: task_info.due_date,
            project_id: task_info.project_id,
            lead_id: task_info.lead_id,
            owner_id: task_info.owner_id,
            count: task_info.count,
            parent_id: task_info.parent_id,
        };

        Ok(task)
    }

    async fn get_tasks(&self, input: GetTasksInput) -> Result<Vec<Task>, SDKError> {
        let where_statement = input.task.to_where_clause();

        let mut query = format!(
            r#"
            SELECT * FROM tasks
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

        let tasks_info = sqlx::query(query.as_str())
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(tasks_info
            .iter()
            .map(|task_info| Task {
                id: task_info.get("id"),
                created_at: task_info.get("created_at"),
                updated_at: task_info.get("updated_at"),
                title: task_info.get("title"),
                description: task_info.get("description"),
                status: TaskStatus::from_optional_str(&task_info.get("status")),
                priority: TaskPriority::from_optional_str(&task_info.get("priority")),
                due_date: task_info.get("due_date"),
                project_id: task_info.get("project_id"),
                lead_id: task_info.get("lead_id"),
                owner_id: task_info.get("owner_id"),
                count: task_info.get("count"),
                parent_id: task_info.get("parent_id"),
            })
            .collect())
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct UpdateTaskInput {
    #[builder(setter(strip_option), default)]
    status: Option<TaskStatus>,
    #[builder(setter(strip_option), default)]
    priority: Option<TaskPriority>,
    #[builder(setter(strip_option), default)]
    title: Option<String>,
    #[builder(setter(strip_option), default)]
    description: Option<String>,
    #[builder(setter(strip_option), default)]
    due_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    project_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    parent_id: Option<Uuid>,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct GetTasksBy {
    #[builder(setter(strip_option), default)]
    owner_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    status: Option<TaskStatus>,
    #[builder(setter(strip_option), default)]
    priority: Option<TaskPriority>,
    #[builder(setter(strip_option), default)]
    title: Option<String>,
    #[builder(setter(strip_option), default)]
    description: Option<String>,
    #[builder(setter(strip_option), default)]
    due_date: Option<DateTime<Utc>>,
    #[builder(setter(strip_option), default)]
    project_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    lead_id: Option<Uuid>,
    #[builder(setter(strip_option), default)]
    parent_id: Option<Uuid>,

    #[builder(setter(strip_option), default)]
    _and: Option<Vec<GetTasksBy>>,
    #[builder(setter(strip_option), default)]
    _or: Option<Vec<GetTasksBy>>,
}

impl GetTasksBy {
    fn to_where_clause(&self) -> String {
        let mut conditions = Vec::new();

        if let Some(owner_id) = &self.owner_id {
            conditions.push(format!("owner_id = {}", owner_id));
        }

        if let Some(status) = &self.status {
            conditions.push(format!("status = '{}'", status));
        }

        if let Some(priority) = &self.priority {
            conditions.push(format!("priority = '{}'", priority));
        }

        if let Some(title) = &self.title {
            conditions.push(format!("title = '{}'", title));
        }

        if let Some(description) = &self.description {
            conditions.push(format!("description = '{}'", description));
        }

        if let Some(due_date) = &self.due_date {
            conditions.push(format!("due_date = '{}'", due_date));
        }

        if let Some(project_id) = &self.project_id {
            conditions.push(format!("project_id = {}", project_id));
        }

        if let Some(lead_id) = &self.lead_id {
            conditions.push(format!("lead_id = {}", lead_id));
        }

        if let Some(parent_id) = &self.parent_id {
            conditions.push(format!("parent_id = {}", parent_id));
        }

        if let Some(ands) = &self._and {
            let and_conditions: Vec<String> =
                ands.iter().map(|and| and.to_where_clause()).collect();
            conditions.push(format!("({})", and_conditions.join(" AND ")));
        }

        if let Some(ors) = &self._or {
            let or_conditions: Vec<String> = ors.iter().map(|or| or.to_where_clause()).collect();
            conditions.push(format!("({})", or_conditions.join(" OR ")));
        }

        conditions.join(" AND ")
    }
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct GetTasksInput {
    task: GetTasksBy,

    #[builder(setter(strip_option), default)]
    sort_by: Option<String>,
    #[builder(setter(strip_option), default)]
    sort_order: Option<SortOrder>,

    #[builder(setter(into, strip_option), default = "Some(100)")]
    limit: Option<i32>,
    #[builder(setter(into, strip_option), default = "Some(0)")]
    offset: Option<i32>,
}
