use async_trait::async_trait;

use crate::{
    backend::loaders::SDKLoaders,
    errors::sdk::SDKError,
    resources::{assets::asset::Asset, members::member::Member, tasks::task::Task},
};

use super::project::Project;

#[async_trait]
pub trait ProjectRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn tasks(&self, loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError>;
    async fn lead(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn assets(&self, loaders: &SDKLoaders) -> Result<Vec<Asset>, SDKError>;
}

#[async_trait]
impl ProjectRelations for Project {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        let data = loaders.member_loader.load_one(self.owner_id).await.unwrap().unwrap();

        Ok(data)
    }

    async fn tasks(&self, _loaders: &SDKLoaders) -> Result<Vec<Task>, SDKError> {
        // let query = query!(
        //     "
        //     SELECT * FROM tasks_by_projects
        //     WHERE project_id = $1
        // ",
        //     self.id
        // );
        todo!()
    }

    async fn lead(&self, _loaders: &SDKLoaders) -> Result<Member, SDKError> {
        todo!()
    }

    async fn assets(&self, _loaders: &SDKLoaders) -> Result<Vec<Asset>, SDKError> {
        todo!()
    }
}
