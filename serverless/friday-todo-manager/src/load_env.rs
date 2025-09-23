use serde_derive::Deserialize;

pub fn load_env_variables() -> EnvVariables {
    dotenv::dotenv().ok();

    let mut config = envy::from_env::<EnvVariables>().unwrap();
    config.is_prod = release_mode();

    config
}

fn release_mode() -> bool {
    !cfg!(debug_assertions)
}

#[derive(Debug, Deserialize)]
pub struct EnvVariables {
    #[serde(skip)]
    pub is_prod: bool,
    // pub secret_manager_url: String,
}
