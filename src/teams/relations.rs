use async_trait::async_trait;

use crate::{errors::sdk::SDKError, members::member::Member, projects::project::Project};

use super::team::Team;

#[async_trait]
pub trait TeamRelations {
    async fn owner(&self) -> Result<Member, SDKError>;
    async fn projects(&self) -> Result<Vec<Project>, SDKError>;
}

#[async_trait]
impl TeamRelations for Team {
    async fn owner(&self) -> Result<Member, SDKError> {
        todo!()
    }

    async fn projects(&self) -> Result<Vec<Project>, SDKError> {
        todo!()
    }
}
