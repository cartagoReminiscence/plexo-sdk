use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::time::OffsetDateTime;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::errors::sdk::SDKError;
use crate::tasks::task::{Task, TaskPriority, TaskStatus};

pub struct DateTimeBridge;

impl DateTimeBridge {
    pub fn to_string(date_time: DateTime<Utc>) -> String {
        date_time.to_rfc3339()
    }

    pub fn from_string(date_time: String) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(&date_time).unwrap()
    }

    pub fn from_offset_date_time(offset_date_time: OffsetDateTime) -> DateTime<Utc> {
        let naive_date_time =
            NaiveDateTime::from_timestamp_millis(offset_date_time.unix_timestamp() * 1000).unwrap();

        // TimeZone::from_utc_datetime(&Utc, &naive_date_time)
        Utc.from_utc_datetime(&naive_date_time)
        // DateTime::<Utc>::from_utc(naive_date_time, Utc)
    }

    pub fn from_date_time(date_time: DateTime<Utc>) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(date_time.timestamp()).unwrap()
    }
}

struct CreateTaskInput {
    owner_id: Uuid,
    status: TaskStatus,
    priority: TaskPriority,
    title: String,
    description: Option<String>,
    due_date: Option<DateTime<Utc>>,
    project_id: Option<Uuid>,
    lead_id: Option<Uuid>,
    parent_id: Option<Uuid>,
}

async fn create_task(pool: Pool<Postgres>, input: CreateTaskInput) -> Result<Task, SDKError> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:password@localhost/test")
    //     .await
    //     .unwrap();

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
        ).fetch_one(&pool)
        .await
        .unwrap();

    todo!()
}
