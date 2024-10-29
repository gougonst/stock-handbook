use crate::models::stock_record_model::StockRecordModel;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct InventoryModel {
    shares: i32,
    buy_price: f64,
    date: DateTime<Utc>,
    current_price: f64,
    fee: i32,
    principal: f64,
}

impl InventoryModel {
    pub fn from_stock_record(record: &StockRecordModel) -> InventoryModel {
        InventoryModel {
            shares: record.get_shares(),
            buy_price: record.get_buy_price(),
            date: record.get_date().clone(),
            current_price: record.get_current_price(),
            fee: record.get_fee(),
            principal: record.get_principal(),
        }
    }

    pub fn add_stock_record(&mut self, record: &StockRecordModel) {
        self.shares += record.get_shares();
        self.principal += record.get_principal();
        self.fee += record.get_fee();
        self.buy_price = self.principal / self.shares as f64;

        self.date = if self.date > record.get_date() {
            self.date
        } else {
            record.get_date()
        };
    }
}
