use async_trait::async_trait;

use crate::{errors::sdk::SDKError, resources::members::member::Member, resources::projects::project::Project};

use super::task::Task;

#[async_trait]
pub trait TaskRelations {
    async fn owner(&self) -> Result<Member, SDKError>;
    async fn project(&self) -> Result<Project, SDKError>;
    async fn lead(&self) -> Result<Member, SDKError>;
    async fn parent(&self) -> Result<Task, SDKError>;
}

#[async_trait]
impl TaskRelations for Task {
    async fn owner(&self) -> Result<Member, SDKError> {
        todo!()
    }

    async fn project(&self) -> Result<Project, SDKError> {
        todo!()
    }

    async fn lead(&self) -> Result<Member, SDKError> {
        todo!()
    }

    async fn parent(&self) -> Result<Task, SDKError> {
        todo!()
    }
}
