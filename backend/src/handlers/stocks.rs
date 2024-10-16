use crate::{app_state::AppState, constants};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct UserInfo {
    username: String,
}

pub async fn list_stocks(info: web::Query<UserInfo>, data: web::Data<AppState>) -> impl Responder {
    info!(
        "Handle 'list_stocks' request with Username: {}",
        info.username
    );

    match data.stock_repo.get_stocks(&info.username).await {
        Ok(stocks) => {
            let resp = serde_json::to_string(&stocks).unwrap();
            HttpResponse::Ok().body(resp)
        }
        Err(e) => {
            error!("Get stocks from DB error: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}
