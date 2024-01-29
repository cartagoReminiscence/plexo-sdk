use async_trait::async_trait;

use crate::{assets::asset::Asset, errors::sdk::SDKError, members::member::Member, projects::project::Project};

#[async_trait]
pub trait AssetRelations {
    async fn owner(&self) -> Result<Member, SDKError>;
    async fn project(&self) -> Result<Project, SDKError>;
}

#[async_trait]
impl AssetRelations for Asset {
    async fn owner(&self) -> Result<Member, SDKError> {
        todo!()
    }

    async fn project(&self) -> Result<Project, SDKError> {
        todo!()
    }
}
