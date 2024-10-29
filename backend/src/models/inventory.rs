use crate::models::stock::Stock;
use chrono::{DateTime, Utc};
use serde::Serialize;
use log::debug;

#[derive(Serialize, Debug, Clone)]
pub struct Inventory {
    shares: i32,
    buy_price: f64,
    date: DateTime<Utc>,
    current_price: f64,
    fee: i32,
    principal: f64,
}

impl Inventory {
    pub fn from_stock(stock: &Stock) -> Inventory {
        Inventory {
            shares: stock.get_shares(),
            buy_price: stock.get_buy_price(),
            date: stock.get_date().clone(),
            current_price: stock.get_current_price(),
            fee: stock.get_fee(),
            principal: stock.get_principal(),
        }
    }

    pub fn add_stock(&mut self, stock: &Stock) {
        self.shares += stock.get_shares();
        self.principal += stock.get_principal();
        self.fee += stock.get_fee();
        self.buy_price = self.principal / self.shares as f64;

        self.date = if self.date > stock.get_date() {
            self.date
        } else {
            stock.get_date()
        };
    }
}
