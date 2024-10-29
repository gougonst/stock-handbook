use crate::models::stock_model::StockModel;
use crate::models::inventory_model::InventoryModel;
use crate::{app_state::AppState, constants};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

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

pub async fn list_inventories(info: web::Query<UserInfo>, data: web::Data<AppState>) -> impl Responder {
    info!("Handle 'list_inventories' request with parameter: {:?}", info);

    match data.stock_repo.get_stocks(&info.username).await {
        Ok(stocks) => {
            let inventories = create_inventories(stocks);
            let resp = serde_json::to_string(&inventories).unwrap();
            HttpResponse::Ok().body(resp)
        }
        Err(e) => {
            error!("Get stocks from DB error: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}

fn create_inventories(stocks: Vec<StockModel>) -> HashMap<String, InventoryModel> {
    let mut grouped_stocks = HashMap::new();

    for stock in stocks {
        let code = stock.get_code();

        if !grouped_stocks.contains_key(code) {
            grouped_stocks.insert(code.to_string(), InventoryModel::from_stock(&stock));
        } else {
            if let Some(grouped_stock) = grouped_stocks.get_mut(code) {
                grouped_stock.add_stock(&stock);
            }
        }
    }
    grouped_stocks
}

pub async fn add_inventory(
    info: web::Json<InventoryInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Handle 'add_inventory' request with parameter: {:?}", info);

    let mut new_stock: StockModel = StockModel::new(
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
