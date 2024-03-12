use uuid::Uuid;

use crate::{errors::sdk::SDKError, resources::members::extensions::MembersExtensionOperations};

use super::engine::SDKEngine;

#[derive(Clone)]
pub struct EngineContext {
    pub member_id: Uuid,
    // organization_id: Option<Uuid>,
    // pub loaders: SDKLoaders,
    _token: String,
}

pub trait Contextualized {
    fn login_with_credentials(
        &self,
        email: &'static str,
        password: &'static str,
    ) -> impl std::future::Future<Output = Result<EngineContext, SDKError>> + Send;

    fn login_with_token(
        &self,
        token: &'static str,
    ) -> impl std::future::Future<Output = Result<EngineContext, SDKError>> + Send;
}

impl Contextualized for SDKEngine {
    async fn login_with_credentials(
        &self,
        email: &'static str,
        password: &'static str,
    ) -> Result<EngineContext, SDKError> {
        let Ok(Some(member)) = self.get_member_by_email(email.to_string()).await else {
            return Err(SDKError::EmailNotFound);
        };

        let Some(password_hash) = member.password_hash.clone() else {
            return Err(SDKError::InvalidPassword);
        };

        if !self.auth.validate_password(password, password_hash.as_str()) {
            return Err(SDKError::InvalidPassword);
        };

        let Ok(session_token) = self.auth.jwt_engine.create_session_token(&member) else {
            return Err(SDKError::InvalidPassword);
        };

        Ok(EngineContext {
            member_id: member.id,
            _token: session_token,
        })
    }

    async fn login_with_token(&self, token: &'static str) -> Result<EngineContext, SDKError> {
        let claims = self.auth.jwt_engine.decode_session_token(token)?;

        Ok(EngineContext {
            member_id: claims.member_id(),
            _token: token.to_string(),
        })
    }
}
