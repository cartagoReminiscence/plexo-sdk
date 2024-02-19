use async_trait::async_trait;

use crate::{errors::sdk::SDKError, resources::tasks::task::Task};

use super::label::Label;

#[async_trait]
pub trait LabelRelations {
    async fn owner(&self) -> Result<Label, SDKError>;
    async fn tasks(&self) -> Result<Vec<Task>, SDKError>;
}

#[async_trait]
impl LabelRelations for Label {
    async fn owner(&self) -> Result<Label, SDKError> {
        todo!()
    }

    async fn tasks(&self) -> Result<Vec<Task>, SDKError> {
        todo!()
    }
}
