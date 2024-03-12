use uuid::Uuid;

use crate::{errors::sdk::SDKError, resources::members::extensions::MembersExtensionOperations};

use super::v2::{Engine, WithoutContext};

#[derive(Default, Clone)]
pub struct EngineContext {
    pub member_id: Uuid,
    _token: String,
}

impl EngineContext {
    pub fn new(member_id: Uuid, token: String) -> EngineContext {
        EngineContext {
            member_id,
            _token: token,
        }
    }

    pub async fn from_email_password(
        engine: &Engine<WithoutContext>,
        email: &'static str,
        password: &'static str,
    ) -> Result<EngineContext, SDKError> {
        let ctx = EngineContext::default();

        let Ok(Some(member)) = engine.get_member_by_email(ctx, email.to_string()).await else {
            return Err(SDKError::EmailNotFound);
        };

        let Some(password_hash) = member.password_hash.clone() else {
            return Err(SDKError::InvalidPassword);
        };

        if !engine.auth.validate_password(password, password_hash.as_str()) {
            return Err(SDKError::InvalidPassword);
        };

        let Ok(session_token) = engine.auth.jwt_engine.create_session_token(&member) else {
            return Err(SDKError::InvalidPassword);
        };

        Ok(EngineContext {
            member_id: member.id,
            _token: session_token,
        })
    }
}
