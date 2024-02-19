use async_trait::async_trait;

use crate::errors::sdk::SDKError;

use super::change::Change;

#[async_trait]
pub trait ChangeRelations {
    async fn owner(&self) -> Result<Change, SDKError>;
    // async fn tasks(&self) -> Result<Vec<Task>, SDKError>;
    // async fn lead(&self) -> Result<Member, SDKError>;
    // async fn assets(&self) -> Result<Vec<Asset>, SDKError>;
}

#[async_trait]
impl ChangeRelations for Change {
    async fn owner(&self) -> Result<Change, SDKError> {
        todo!()
    }
}
