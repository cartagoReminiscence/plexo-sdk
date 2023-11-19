use async_trait::async_trait;

use crate::{errors::sdk::SDKError, members::member::Member, projects::project::Project};

use super::task::Task;

#[async_trait]
trait TaskRelations {
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
