use std::str::FromStr;

use chrono::{DateTime, Utc};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub description: Option<String>,

    pub owner_id: Uuid,

    pub status: TaskStatus,
    pub priority: TaskPriority,

    pub due_date: Option<DateTime<Utc>>,

    pub project_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,

    pub count: i32,

    pub parent_id: Option<Uuid>,
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum TaskStatus {
    None,
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

impl TaskStatus {
    pub fn from_optional_str(status: &Option<String>) -> Self {
        match status {
            Some(status) => TaskStatus::from_str(status.as_str()).unwrap(),
            None => TaskStatus::None,
        }
    }
}

#[derive(Clone, Debug, Display, EnumString)]
pub enum TaskPriority {
    None,
    Low,
    Medium,
    High,
    Urgent,
}

impl TaskPriority {
    pub fn from_optional_str(priority: &Option<String>) -> Self {
        match priority {
            Some(priority) => TaskPriority::from_str(priority.as_str()).unwrap(),
            None => TaskPriority::None,
        }
    }
}
