use uuid::Uuid;

use crate::errors::sdk::SDKError;

#[derive(Clone)]
pub struct EngineContext {
    pub member_id: Uuid,
    // organization_id: Option<Uuid>,
    _token: String,
}

impl EngineContext {
    pub async fn from_credentials(_email: &'static str, _password: &'static str) -> Result<EngineContext, SDKError> {
        // let Ok(Some(member)) = plexo_engine.engine.get_member_by_email(email.clone()).await else {
        //     return Err(PlexoAppError::EmailNotFound.into());
        // };

        // let Some(password_hash) = member.password_hash.clone() else {
        //     return Err(PlexoAppError::InvalidPassword.into());
        // };

        // if !plexo_engine
        //     .auth
        //     .validate_password(password.as_str(), password_hash.as_str())
        // {
        //     return Err(PlexoAppError::InvalidPassword.into());
        // };

        // let Ok(session_token) = plexo_engine.auth.jwt_engine.create_session_token(&member) else {
        //     return Err(PlexoAppError::InvalidPassword.into());
        // };

        Ok(EngineContext {
            member_id: Uuid::new_v4(),
            _token: String::default(),
        })
    }

    pub async fn from_token(_token: &'static str) -> Result<EngineContext, SDKError> {
        todo!()
    }
}
