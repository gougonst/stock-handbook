use crate::models::stock_record_model::StockRecordModel;
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

    match data.record_repo.get_stock_records(&info.username).await {
        Ok(records) => {
            let inventories = create_inventories(records);
            let resp = serde_json::to_string(&inventories).unwrap();
            HttpResponse::Ok().body(resp)
        }
        Err(e) => {
            error!("Get stock records from DB error: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}

fn create_inventories(records: Vec<StockRecordModel>) -> HashMap<String, InventoryModel> {
    let mut inventories = HashMap::new();

    for record in records {
        let code = record.get_code();

        if !inventories.contains_key(code) {
            inventories.insert(code.to_string(), InventoryModel::from_stock_record(&record));
        } else {
            if let Some(inventory) = inventories.get_mut(code) {
                inventory.add_stock_record(&record);
            }
        }
    }
    inventories
}

pub async fn add_inventory(
    info: web::Json<InventoryInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Handle 'add_inventory' request with parameter: {:?}", info);

    let mut new_record: StockRecordModel = StockRecordModel::new(
        None,
        info.username.clone(),
        info.code.clone(),
        info.shares,
        info.buy_price,
        info.date,
        info.current_price,
    );
    match data.record_repo.add_stock_record(&new_record).await {
        Ok(oid) => {
            new_record.set_id(Some(oid));
            let resp = serde_json::to_string(&new_record).unwrap();
            HttpResponse::Ok().body(resp)
        }
        Err(e) => {
            error!("Add stock record to DB failed: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}
