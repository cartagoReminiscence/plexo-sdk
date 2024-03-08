use uuid::Uuid;

use crate::errors::sdk::SDKError;

#[derive(Clone)]
pub struct EngineContext {
    pub member_id: Uuid,
    // organization_id: Option<Uuid>,
}

impl EngineContext {
    pub async fn from_credentials(_email: &'static str, _password: &'static str) -> Result<EngineContext, SDKError> {
        // TODO: implement

        Ok(EngineContext {
            member_id: Uuid::new_v4(),
        })
    }

    pub async fn from_token(_token: &'static str) -> EngineContext {
        todo!()
    }
}
