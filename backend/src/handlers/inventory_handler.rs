use crate::models::inventory_model::InventoryModel;
use crate::models::stock_error::StockError;
use crate::models::stock_record_model::{StockRecordAction, StockRecordModel};
use crate::{app_state::AppState, constants};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct StockRecordInfo {
    username: String,
    code: String,
    shares: i32,
    transaction_price: f64,
    date: DateTime<Utc>,
    current_price: f64,
}

pub async fn list_inventories(
    info: web::Query<UserInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Handle 'list_inventories' request with parameter: {:?}",
        info
    );

    match data.record_repo.get_stock_records(&info.username).await {
        Ok(records) => {
            if let Ok(inventories) = create_inventories(records) {
                let resp = serde_json::to_string(&inventories).unwrap();
                HttpResponse::Ok().body(resp)
            } else {
                HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
            }
        }
        Err(e) => {
            error!("Get stock records from DB error: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}

fn create_inventories(records: Vec<StockRecordModel>) -> Result<HashMap<String, InventoryModel>, StockError> {
    let mut inventories = HashMap::new();

    for record in records {
        let code = record.get_code();

        if !inventories.contains_key(code) {
            inventories.insert(code.to_string(), InventoryModel::from_stock_record(&record));
        } else {
            if let Some(inventory) = inventories.get_mut(code) {
                inventory.update_stock_record(&record);
            }
        }
    }

    // Remove the inventories which shares is 0
    let codes_to_remove: Vec<_> = inventories.iter()
        .filter(|(_, inventory)| inventory.get_shares() == 0)
        .map(|(code, _)| code.clone())
        .collect();

    for code in codes_to_remove {
        inventories.remove(&code);
    }

    // If there is inventory's shares < 0, it should raise error
    let code_with_error: Vec<_> = inventories.iter()
        .filter(|(_, inventory)| inventory.get_shares() < 0)
        .map(|(code, _)| code)
        .collect();
    if code_with_error.len() > 0 {
        return Err(StockError::TransactionError);
    }

    Ok(inventories)
}

pub async fn add_inventory(
    info: web::Json<StockRecordInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Handle 'add_inventory' request with parameter: {:?}", info);

    let mut new_record: StockRecordModel = StockRecordModel::new(
        None,
        StockRecordAction::Add,
        info.username.clone(),
        info.code.clone(),
        info.shares,
        info.transaction_price,
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

pub async fn reduce_inventory(
    info: web::Json<StockRecordInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Handle 'reduce_inventory' request with parameter: {:?}", info);

    let new_record: StockRecordModel = StockRecordModel::new(
        None, 
        StockRecordAction::Delete, 
        info.username.clone(), 
        info.code.clone(), 
        info.shares, 
        info.transaction_price, 
        info.date, 
        info.current_price, 
    );
    match data.record_repo.add_stock_record(&new_record).await {
        Ok(_) => {
            HttpResponse::Ok().body(constants::HTTP_OK)
        }
        Err(e) => {
            error!("Add stock record to DB failed: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}
