use serde_derive::Deserialize;

pub fn load_env_variables() -> EnvVariables {
    dotenv::dotenv().ok();

    let is_prod = release_mode();
    let mut config = if is_prod {
        envy::from_env::<EnvVariables>().expect("Error loading .env file or environment variables")
    } else {
        // Valores padrão para desenvolvimento
        EnvVariables {
            is_prod: false,
            todo_api_base_url: "https://k8s.z33p.com/api/friday-todo-manager".to_string(),
        }
    };

    config.is_prod = is_prod;

    config
}

fn release_mode() -> bool {
    !cfg!(debug_assertions)
}

#[derive(Debug, Deserialize)]
pub struct EnvVariables {
    #[serde(skip)]
    pub is_prod: bool,
    pub todo_api_base_url: String,
}
