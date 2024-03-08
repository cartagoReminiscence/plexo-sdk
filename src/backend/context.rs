use uuid::Uuid;

pub struct EngineContext {
    pub member_id: Uuid,
    // organization_id: Option<Uuid>,
}

impl EngineContext {
    pub fn from_credentials(_email: &'static str, _password: &'static str) -> EngineContext {
        todo!()
    }

    pub fn from_token(_token: &'static str) -> EngineContext {
        todo!()
    }
}
