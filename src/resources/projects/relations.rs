use async_trait::async_trait;
use sqlx::query;

use crate::{
    backend::engine::SDKEngine,
    errors::sdk::SDKError,
    resources::{
        assets::asset::Asset,
        members::{member::Member, operations::MemberCrudOperations},
        tasks::task::Task,
    },
};

use super::project::Project;

#[async_trait]
pub trait ProjectRelations {
    async fn owner(&self, engine: &SDKEngine) -> Result<Member, SDKError>;
    async fn tasks(&self, engine: &SDKEngine) -> Result<Vec<Task>, SDKError>;
    async fn lead(&self, engine: &SDKEngine) -> Result<Member, SDKError>;
    async fn assets(&self, engine: &SDKEngine) -> Result<Vec<Asset>, SDKError>;
}

#[async_trait]
impl ProjectRelations for Project {
    async fn owner(&self, engine: &SDKEngine) -> Result<Member, SDKError> {
        engine.get_member(self.owner_id).await
    }

    async fn tasks(&self, engine: &SDKEngine) -> Result<Vec<Task>, SDKError> {
        // let query = query!(
        //     "
        //     SELECT * FROM tasks_by_projects
        //     WHERE project_id = $1
        // ",
        //     self.id
        // );
        todo!()
    }

    async fn lead(&self, engine: &SDKEngine) -> Result<Member, SDKError> {
        todo!()
    }

    async fn assets(&self, engine: &SDKEngine) -> Result<Vec<Asset>, SDKError> {
        todo!()
    }
}
