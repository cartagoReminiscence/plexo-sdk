use std::env::var;

pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct SDKConfig {
    pub database_url: String,
    pub llm_api_key: String,
    pub llm_model_name: String,
    pub jwt_access_token_secret: String,
    pub jwt_refresh_token_secret: String,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub github_redirect_url: Option<String>,
}

impl SDKConfig {
    pub fn from_env() -> SDKConfig {
        let database_url = var("DATABASE_URL").unwrap();
        let llm_api_key = var("OPENAI_API_KEY").unwrap();
        let llm_model_name = var("OPENAI_MODEL_NAME").unwrap_or("gpt-3.5-turbo".to_string());

        let jwt_access_token_secret = var("JWT_ACCESS_TOKEN_SECRET").unwrap_or("secret".to_string());
        let jwt_refresh_token_secret = var("JWT_REFRESH_TOKEN_SECRET").unwrap_or(jwt_access_token_secret.clone());
        let github_client_id = var("GITHUB_CLIENT_ID").ok();
        let github_client_secret = var("GITHUB_CLIENT_SECRET").ok();
        let github_redirect_url = var("GITHUB_REDIRECT_URL").ok();

        SDKConfig {
            database_url,
            llm_api_key,
            llm_model_name,
            jwt_access_token_secret,
            jwt_refresh_token_secret,
            github_client_id,
            github_client_secret,
            github_redirect_url,
        }
    }
}
