use async_trait::async_trait;

use crate::{
    backend::loaders::SDKLoaders, errors::sdk::SDKError, resources::members::member::Member,
    resources::projects::project::Project,
};

use super::task::Task;

#[async_trait]
pub trait TaskRelations {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError>;
    async fn project(&self) -> Result<Project, SDKError>;
    async fn lead(&self) -> Result<Member, SDKError>;
    async fn parent(&self) -> Result<Task, SDKError>;
}

#[async_trait]
impl TaskRelations for Task {
    async fn owner(&self, loaders: &SDKLoaders) -> Result<Member, SDKError> {
        let data = loaders.member_loader.load_one(self.owner_id).await.unwrap().unwrap();

        Ok(data)
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
