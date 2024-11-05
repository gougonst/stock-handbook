use crate::constants::{TRANSACTION_BUY, TRANSACTION_SELL};
use crate::models::stock_error::StockError;
use crate::models::stock_record_model::{StockRecordAction, StockRecordModel};
use crate::{app_state::AppState, constants};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::{error, info};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    username: String,
    transaction_type: String,
}

pub async fn list_history(info: web::Query<UserInfo>, data: web::Data<AppState>) -> impl Responder {
    info!("Handle 'list_history' request with parameter: {:?}", info);

    match data.record_repo.get_stock_records(&info.username).await {
        Ok(records) => {
            let transaction_type = match info.transaction_type.as_str() {
                TRANSACTION_SELL => Ok(&StockRecordAction::Delete),
                TRANSACTION_BUY => Ok(&StockRecordAction::Add),
                _ => {
                    error!("No such transaction type: {}", &info.transaction_type);
                    Err(StockError::TransactionTypeError)
                }
            };

            match transaction_type {
                Ok(transaction_type) => {
                    let sell_records: Vec<_> = records
                        .into_iter()
                        .filter(|record| record.get_action() == transaction_type)
                        .collect();
                    let resp = serde_json::to_string(&sell_records).unwrap();
                    HttpResponse::Ok().body(resp)
                }
                Err(_) => HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR),
            }
        }
        Err(e) => {
            error!("Get stock records error from DB: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}
