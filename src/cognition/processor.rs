use async_openai::{config::OpenAIConfig, Client};
use async_trait::async_trait;
use chrono::Local;
use sqlx::{Pool, Postgres};

use crate::errors::sdk::SDKError;

use super::{
    operations::{CognitionOperations, SubdivideTaskInput, TaskSuggestionInput, TaskSuggestionResult},
    suggestions::CognitionCapabilities,
};

pub struct Cognition {
    client: Client<OpenAIConfig>,
    model_name: String,
    pub pool: Box<Pool<Postgres>>,
}

impl Cognition {
    pub async fn new(pool: Box<Pool<Postgres>>, api_key: String, model_name: String) -> Self {
        let config = OpenAIConfig::default().with_api_key(api_key);

        let client = Client::with_config(config);

        Self {
            client,
            model_name,
            pool,
        }
    }

    pub fn client(&self) -> &Client<OpenAIConfig> {
        &self.client
    }

    pub fn model_name(&self) -> &str {
        &self.model_name
    }
}

#[async_trait]
impl CognitionOperations for Cognition {
    async fn get_suggestions(&self, input: TaskSuggestionInput) -> Result<TaskSuggestionResult, SDKError> {
        let tasks_fingerprints = self.acquire_tasks_fingerprints(10, input.project_id).await;

        let system_message =
            "The user pass to you a list of tasks and you should predict the following based on the input of the user.
        Please return only a valid json with the following struct {
                title: String,
                description: String,
                status: TaskStatus,
                priority: TaskPriority,
                due_date: DateTime<Utc>
        }"
            .to_string();

        let user_message = format!(
            "
            Current Time:
            {}

            Current Tasks Context: 
            {}
            
            With the above context, complete the following task, only fill the <suggest> fields:
            {}",
            Local::now(),
            tasks_fingerprints.join("\n\n"),
            Self::calculate_task_suggestion_fingerprint(input),
        );

        let result = self.chat_completion(system_message, user_message).await;

        let suggestion_result: TaskSuggestionResult = serde_json::from_str(&result)?;

        Ok(suggestion_result)
    }

    async fn subdivide_task(&self, _input: SubdivideTaskInput) -> Result<Vec<TaskSuggestionResult>, SDKError> {
        todo!()
    }
}
