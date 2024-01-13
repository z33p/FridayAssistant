use serde_derive::Deserialize;

pub fn load_env_variables() -> EnvVariables {
    if release_mode() {
        dotenv::from_filename(".env.prod").ok();
    } else {
        dotenv::dotenv().ok();
    }
    
    let config = envy::from_env::<EnvVariables>().unwrap();

    config
}

fn release_mode() -> bool {
    !cfg!(debug_assertions)
}

#[derive(Debug, Deserialize)]
pub struct EnvVariables {
    pub is_prod: bool,
    pub open_ai_api_key: String,
    pub news_api_key: String,

    pub rabbit_host: String,
    pub rabbit_port: u16,
    pub rabbit_user: String,
    pub rabbit_password: String,

    pub queue_newsletter_message_broker_response: String
}
