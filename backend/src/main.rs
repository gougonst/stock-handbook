use actix_web::{web, App, HttpServer};
use app_state::AppState;
use database::mongo_repository::MongoUserRepository;
use std::env;
use std::sync::Arc;
use mongodb::{Client, Database};
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
    let db = init_db().await.unwrap_or_else(|e| {
        panic!("{}", format!("{}: {}", constants::INIT_DB_ERR, e));
    });
    let user_repo = Arc::new(MongoUserRepository::new(db));
    let app_state = AppState {
        user_repo
    };
    let data = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(routes::auth::auth_scope)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
