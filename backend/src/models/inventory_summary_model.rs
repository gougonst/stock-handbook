use super::stock_error::StockError;
use super::stock_record_model::{StockRecordAction, StockRecordModel};
use chrono::{DateTime, Utc};
use log::error;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
struct StockInventoryModel {
    shares: i32,
    transaction_price: f64,
    date: DateTime<Utc>,
    current_price: f64,
    fee: i32,
    principal: f64,
}

impl StockInventoryModel {
    fn get_shares(&self) -> i32 {
        self.shares
    }

    fn from_stock_record(record: &StockRecordModel) -> StockInventoryModel {
        StockInventoryModel {
            shares: record.get_shares(),
            transaction_price: record.get_transaction_price(),
            date: record.get_date().clone(),
            current_price: record.get_current_price(),
            fee: record.get_fee(),
            principal: record.get_principal(),
        }
    }

    fn update_stock_record(&mut self, record: &StockRecordModel) {
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
        self.principal -=
            StockRecordModel::calc_principal(record.get_shares(), self.transaction_price);
        self.date = if self.date > record.get_date() {
            self.date
        } else {
            record.get_date()
        };
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct InventorySummaryModel {
    inventories: HashMap<String, StockInventoryModel>,
}

impl InventorySummaryModel {
    pub fn from_stock_records(
        records: Vec<StockRecordModel>,
    ) -> Result<InventorySummaryModel, StockError> {
        let mut inventories = HashMap::new();

        for record in records {
            let code = record.get_code();

            if !inventories.contains_key(code) {
                inventories.insert(
                    code.to_string(),
                    StockInventoryModel::from_stock_record(&record),
                );
            } else {
                if let Some(inventory) = inventories.get_mut(code) {
                    inventory.update_stock_record(&record);
                }
            }
        }

        // Remove the inventories which shares is 0
        let codes_to_remove: Vec<_> = inventories
            .iter()
            .filter(|(_, inventory)| inventory.get_shares() == 0)
            .map(|(code, _)| code.clone())
            .collect();

        for code in codes_to_remove {
            inventories.remove(&code);
        }

        // If there is inventory's shares < 0, it should raise error
        let code_with_error: Vec<_> = inventories
            .iter()
            .filter(|(_, inventory)| inventory.get_shares() < 0)
            .map(|(code, _)| code)
            .collect();
        if code_with_error.len() > 0 {
            return Err(StockError::TransactionError);
        }

        Ok(InventorySummaryModel { inventories })
    }
}
