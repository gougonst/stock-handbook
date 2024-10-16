use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use app_state::AppState;
use database::stock_repository::StockRepository;
use database::user_repository::UserRepository;
use env_logger::Env;
use log::info;
use mongodb::{Client, Database};
use std::env;
use std::sync::Arc;
mod app_state;
mod constants;
mod database;
mod handlers;
mod models;
mod routes;

async fn init_db() -> Result<Arc<Database>, Box<dyn std::error::Error>> {
    let uri = match env::var(constants::MONGODB_CONN_STR_ENV) {
        Ok(uri) => uri,
        _ => {
            panic!("{}", constants::GET_MONGODB_CONN_STR_ENV_FAIL);
        }
    };
    let client = Client::with_uri_str(uri).await?;
    // Get a handle on the movies collection
    Ok(Arc::new(client.database(constants::DATABASE_NAME)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Init database
    let db = init_db().await.unwrap_or_else(|e| {
        panic!("{}", format!("{}: {}", constants::INIT_DB_ERR, e));
    });
    let user_repo = Arc::new(UserRepository::new(Arc::clone(&db)));
    let stock_repo = Arc::new(StockRepository::new(Arc::clone(&db)));
    let app_state = AppState {
        user_repo,
        stock_repo,
    };
    let data = web::Data::new(app_state);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(constants::CORS_DOMAIN)
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT]),
            )
            .app_data(data.clone())
            .configure(routes::auth::auth_scope)
            .configure(routes::stock::stock_scope)
    })
    .bind(("0.0.0.0", 8081))?;

    info!("Server is running on 'http://0.0.0.0:8081'");
    server.run().await
}
