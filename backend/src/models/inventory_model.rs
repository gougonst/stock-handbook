use super::stock_error::StockError;
use super::stock_record_model::{StockRecordAction, StockRecordModel};
use chrono::{DateTime, Utc};
use log::error;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct InventoryModel {
    shares: i32,
    transaction_price: f64,
    date: DateTime<Utc>,
    current_price: f64,
    fee: i32,
    principal: f64,
}

impl InventoryModel {
    pub fn get_shares(&self) -> i32 {
        self.shares
    }

    pub fn from_stock_record(record: &StockRecordModel) -> InventoryModel {
        InventoryModel {
            shares: record.get_shares(),
            transaction_price: record.get_transaction_price(),
            date: record.get_date().clone(),
            current_price: record.get_current_price(),
            fee: record.get_fee(),
            principal: record.get_principal(),
        }
    }

    pub fn update_stock_record(&mut self, record: &StockRecordModel) {
        let action = record.get_action();
        match action {
            StockRecordAction::Add => {
                self.increase_stock_record(record);
            }
            StockRecordAction::Delete => {
                self.decrease_stock_record(record);
            }
        }
    }

    fn increase_stock_record(&mut self, record: &StockRecordModel) {
        self.shares += record.get_shares();
        self.principal += record.get_principal();
        self.fee += record.get_fee();
        self.transaction_price = self.principal / self.shares as f64;
        self.date = if self.date > record.get_date() {
            self.date
        } else {
            record.get_date()
        };
    }

    fn decrease_stock_record(&mut self, record: &StockRecordModel) {
        self.shares -= record.get_shares();
        self.principal -= StockRecordModel::calc_principal(record.get_shares(), self.transaction_price);
        self.date = if self.date > record.get_date() {
            self.date
        } else {
            record.get_date()
        };
    }
}
