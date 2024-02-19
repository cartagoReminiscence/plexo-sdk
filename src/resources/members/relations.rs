use async_trait::async_trait;

use crate::{errors::sdk::SDKError, resources::projects::project::Project, resources::tasks::task::Task};

use super::member::Member;

#[async_trait]
pub trait MemberRelations {
    async fn projects(&self) -> Result<Vec<Project>, SDKError>;
    async fn tasks(&self) -> Result<Vec<Task>, SDKError>;
}

#[async_trait]
impl MemberRelations for Member {
    async fn projects(&self) -> Result<Vec<Project>, SDKError> {
        todo!()
    }

    async fn tasks(&self) -> Result<Vec<Task>, SDKError> {
        todo!()
    }
}
