use async_trait::async_trait;

use crate::{
    errors::sdk::SDKError, resources::assets::asset::Asset, resources::members::member::Member,
    resources::projects::project::Project,
};

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
