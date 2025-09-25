use actix_web::{get, App, HttpServer, Responder};
use dotenv::dotenv;
use load_env::{load_env_variables, EnvVariables};
use once_cell::sync::Lazy;
use tracing::Level;

use crate::todo_mod::{task_controller, todo_list_controller};

mod business_response;
mod load_env;
mod microsoft_graph_mod;
mod oauth_mod;
mod openapi;
mod secret_manager_mod;
mod todo_mod;

extern crate dotenv;

static ENV_CONFIG: Lazy<EnvVariables> = Lazy::new(|| load_env_variables());

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    logging_init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(todo_list_controller::get_todo_list)
            .service(todo_list_controller::get_all_todo_lists)
            .service(todo_list_controller::create_todo_list)
            .service(todo_list_controller::update_todo_list)
            .service(todo_list_controller::delete_todo_list)
            .service(task_controller::get_all_tasks)
            .service(task_controller::get_task)
            .service(task_controller::create_task)
            .service(task_controller::update_task)
            .service(task_controller::delete_task)
            .service(openapi::swagger_config())
    })
    .workers(4)
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}

fn logging_init() {
    let log_level = if ENV_CONFIG.is_prod {
        Level::INFO
    } else {
        Level::DEBUG
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();
}
