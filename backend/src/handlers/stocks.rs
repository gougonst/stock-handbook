use crate::models::stock::Stock;
use crate::{app_state::AppState, constants};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryInfo {
    username: String,
    code: String,
    shares: i32,
    buy_price: f64,
    date: DateTime<Utc>,
    current_price: f64,
}

#[derive(Debug, Deserialize)]
pub struct StockIdInfo {
    id: String,
}

pub async fn list_stocks(info: web::Query<UserInfo>, data: web::Data<AppState>) -> impl Responder {
    info!("Handle 'list_stocks' request with parameter: {:?}", info);

    match data.stock_repo.get_stocks(&info.username).await {
        Ok(stocks) => {
            debug!("Freddy, inventories: {:?}", stocks);
            let resp = serde_json::to_string(&stocks).unwrap();
            HttpResponse::Ok().body(resp)
        }
        Err(e) => {
            error!("Get stocks from DB error: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}

pub async fn add_stock(
    info: web::Json<InventoryInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Handle 'add_stock' request with parameter: {:?}", info);

    let mut new_stock: Stock = Stock::new(
        None,
        info.username.clone(),
        info.code.clone(),
        info.shares,
        info.buy_price,
        info.date,
        info.current_price,
    );
    match data.stock_repo.add_stock(&new_stock).await {
        Ok(oid) => {
            new_stock.set_id(Some(oid));
            let resp = serde_json::to_string(&new_stock).unwrap();
            HttpResponse::Ok().body(resp)
        }
        Err(e) => {
            error!("Add stock to DB failed: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}

pub async fn delete_stock(
    info: web::Json<StockIdInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Handle 'delete_stock' request with parameter: {:?}", info);

    match data.stock_repo.delete_stocks(&info.id).await {
        Ok(true) => {
            debug!("Delete stock with id: {} successfully", info.id);
            HttpResponse::Ok()
        }
        Ok(false) => {
            error!("Delete stock failed, cannot find id: {}", info.id);
            HttpResponse::NotFound()
        }
        Err(e) => {
            error!("Delete stock from DB failed: {}", e);
            HttpResponse::InternalServerError()
        }
    }
}
