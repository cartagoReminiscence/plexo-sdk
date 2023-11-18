use chrono::{DateTime, Utc};
use strum_macros::Display;
use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

    title: String,
    description: Option<String>,

    owner_id: Uuid,

    status: TaskStatus,
    priority: TaskPriority,

    due_date: Option<DateTime<Utc>>,

    project_id: Option<Uuid>,
    lead_id: Option<Uuid>,

    count: i32,

    parent_id: Option<Uuid>,
}

#[derive(Display, Debug)]
pub enum TaskStatus {
    None,
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

#[derive(Display, Debug)]
pub enum TaskPriority {
    None,
    Low,
    Medium,
    High,
    Urgent,
}
