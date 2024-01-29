use async_trait::async_trait;

use crate::{assets::asset::Asset, errors::sdk::SDKError, members::member::Member, tasks::task::Task};

use super::project::Project;

#[async_trait]
pub trait ProjectRelations {
    async fn owner(&self) -> Result<Member, SDKError>;
    async fn tasks(&self) -> Result<Vec<Task>, SDKError>;
    async fn lead(&self) -> Result<Member, SDKError>;
    async fn assets(&self) -> Result<Vec<Asset>, SDKError>;
}

#[async_trait]
impl ProjectRelations for Project {
    async fn owner(&self) -> Result<Member, SDKError> {
        todo!()
    }

    async fn tasks(&self) -> Result<Vec<Task>, SDKError> {
        todo!()
    }

    async fn lead(&self) -> Result<Member, SDKError> {
        todo!()
    }

    async fn assets(&self) -> Result<Vec<Asset>, SDKError> {
        todo!()
    }
}
