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
    pub oauth_client_id: String,
    pub oauth_client_secret: String,
    pub database_url: String
}
