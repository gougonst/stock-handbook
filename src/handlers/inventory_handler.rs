use crate::models::inventory_summary_model::InventorySummaryModel;
use crate::models::stock_record_model::{StockRecordAction, StockRecordModel};
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
pub struct StockRecordInfo {
    username: String,
    code: String,
    shares: i32,
    transaction_price: f64,
    date: DateTime<Utc>,
    current_price: f64,
}

pub async fn list_inventory_summary(
    info: web::Query<UserInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!(
        "Handle 'list_inventory_summary' request with parameter: {:?}",
        info
    );

    match data.record_repo.get_stock_records(&info.username).await {
        Ok(records) => {
            debug!("{:?}", records);
            if let Ok(inventories) = InventorySummaryModel::from_stock_records(records) {
                debug!("{:?}", inventories);
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

pub async fn buy_stock(
    info: web::Json<StockRecordInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Handle 'buy_stock' request with parameter: {:?}", info);

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

pub async fn sell_stock(
    info: web::Json<StockRecordInfo>,
    data: web::Data<AppState>,
) -> impl Responder {
    info!("Handle 'sell_stock' request with parameter: {:?}", info);

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
        Ok(_) => HttpResponse::Ok().body(constants::HTTP_OK),
        Err(e) => {
            error!("Add stock record to DB failed: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}
