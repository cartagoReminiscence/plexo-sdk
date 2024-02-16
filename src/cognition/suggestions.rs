use std::str::FromStr;

use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use sqlx::query;
use uuid::Uuid;

use super::{operations::TaskSuggestionInput, processor::Cognition};
use crate::tasks::task::{Task, TaskPriority, TaskStatus};

pub trait CognitionCapabilities {
    fn chat_completion(
        &self,
        system_message: String,
        user_message: String,
    ) -> impl std::future::Future<Output = String> + Send;
    fn acquire_tasks_fingerprints(
        &self,
        number_of_tasks: u32,
        project_id: Option<Uuid>,
    ) -> impl std::future::Future<Output = Vec<String>> + Send;

    fn calculate_task_fingerprint(task: Task) -> String;
    fn calculate_task_suggestion_fingerprint(task_suggestion: TaskSuggestionInput) -> String;
}

impl CognitionCapabilities for Cognition {
    async fn chat_completion(&self, system_message: String, user_message: String) -> String {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model(self.model_name().to_string())
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_message)
                    .build()
                    .unwrap()
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_message)
                    .build()
                    .unwrap()
                    .into(),
            ])
            .build()
            .unwrap();

        let response = self.client().chat().create(request).await.unwrap();

        response.choices.first().unwrap().message.content.clone().unwrap()
    }

    fn calculate_task_fingerprint(task: Task) -> String {
        serde_json::to_string(&task).unwrap()
    }

    fn calculate_task_suggestion_fingerprint(task_suggestion: TaskSuggestionInput) -> String {
        format!(
            "Task Title: {}
        Task Description: {}
        Task Status: {}
        Task Priority: {}
        Task Due Date: {}",
            task_suggestion.title.unwrap_or("<suggest>".to_string()),
            task_suggestion.description.unwrap_or("<suggest>".to_string()),
            task_suggestion
                .status
                .map(|s| s.to_string())
                .unwrap_or("<suggest>".to_string()),
            task_suggestion
                .priority
                .map(|p| p.to_string())
                .unwrap_or("<suggest>".to_string()),
            task_suggestion
                .due_date
                .map(|d| d.to_rfc3339())
                .unwrap_or("<suggest>".to_string()),
        )
    }

    async fn acquire_tasks_fingerprints(&self, _number_of_tasks: u32, _project_id: Option<Uuid>) -> Vec<String> {
        let tasks = query!(
            r#"
        SELECT *
        FROM tasks
        LIMIT 10
        "#,
        )
        .fetch_all(&*self.pool)
        .await
        .unwrap();

        tasks
            .iter()
            .map(|r| Task {
                id: r.id,
                created_at: r.created_at,
                updated_at: r.updated_at,
                title: r.title.clone(),
                description: r.description.clone(),
                status: r
                    .status
                    .clone()
                    .and_then(|a| TaskStatus::from_str(&a).ok())
                    .unwrap_or_default(),
                priority: r
                    .priority
                    .clone()
                    .and_then(|a| TaskPriority::from_str(&a).ok())
                    .unwrap_or_default(),
                due_date: r.due_date,
                project_id: r.project_id,
                lead_id: r.lead_id,
                owner_id: r.owner_id,
                count: r.count,
                parent_id: r.parent_id,
            })
            .map(Self::calculate_task_fingerprint)
            .collect::<Vec<String>>()
    }
}
