use bson::{from_bson, Bson};
use chrono::prelude::*;
use chrono::Utc;
use log::debug;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Serialize, Debug)]
pub struct StockModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    username: String,
    code: String,
    shares: i32,
    buy_price: f64,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    date: DateTime<Utc>,
    current_price: f64,
    fee: i32,
    principal: f64,
}

impl StockModel {
    pub fn new(
        id: Option<String>,
        username: String,
        code: String,
        shares: i32,
        buy_price: f64,
        date: DateTime<Utc>,
        current_price: f64,
    ) -> StockModel {
        StockModel {
            id,
            username,
            code,
            shares,
            buy_price,
            date,
            current_price,
            fee: StockModel::calc_fee(shares, buy_price),
            principal: StockModel::calc_principal(shares, buy_price),
        }
    }

    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn get_current_price(&self) -> f64 {
        self.current_price
    }

    pub fn get_date(&self) -> DateTime<Utc> {
        self.date
    }

    pub fn get_fee(&self) -> i32 {
        self.fee
    }

    pub fn get_principal(&self) -> f64 {
        self.principal
    }

    pub fn get_shares(&self) -> i32 {
        self.shares
    }

    pub fn set_shares(&mut self, shares: i32) {
        self.shares = shares;
        self.principal = StockModel::calc_principal(shares, self.buy_price);
        self.fee = StockModel::calc_fee(shares, self.buy_price);
    }

    pub fn get_buy_price(&self) -> f64 {
        self.buy_price
    }

    pub fn set_buy_price(&mut self, buy_price: f64) {
        self.buy_price = buy_price;
        self.principal = StockModel::calc_principal(self.shares, buy_price);
        self.fee = StockModel::calc_fee(self.shares, buy_price);
    }

    pub fn calc_principal(shares: i32, buy_price: f64) -> f64 {
        shares as f64 * buy_price
    }

    fn calc_fee(shares: i32, buy_price: f64) -> i32 {
        let fee = (StockModel::calc_principal(shares, buy_price) * 0.001425).trunc() as i32;
        match fee <= 20 {
            true => 20,
            false => fee,
        }
    }
}

impl<'de> Deserialize<'de> for StockModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let doc = bson::Document::deserialize(deserializer)?;

        debug!("Doc: {:?}", doc);

        let id = doc
            .get_object_id(constants::STOCK_COLL_ID_COL)
            .map(|oid| oid.to_hex())
            .ok();
        let buy_price = doc
            .get_f64(constants::STOCK_COLL_BUY_PRICE_COL)
            .map_err(|e| serde::de::Error::custom(format!("Failed to get buy price: {e}")))?;
        let code = doc
            .get_str(constants::STOCK_COLL_CODE_COL)
            .map(|s| s.to_string())
            .map_err(|e| serde::de::Error::custom(format!("Failed to get code: {e}")))?;
        let current_price = doc
            .get_f64(constants::STOCK_COLL_CURRENT_PRICE_COL)
            .map_err(|e| serde::de::Error::custom(format!("Failed to get current price: {e}")))?;
        let date = doc
            .get_datetime(constants::STOCK_COLL_DATE_COL)
            .map(|dt| dt.to_chrono())
            .map_err(|e| serde::de::Error::custom(format!("Failed to get date: {e}")))?;
        let shares = doc
            .get_i32(constants::STOCK_COLL_SHARES_COL)
            .map_err(|e| serde::de::Error::custom(format!("Failed to get shares: {e}")))?;
        let username = doc
            .get_str(constants::STOCK_COLL_USERNAME_COL)
            .map(|s| s.to_string())
            .map_err(|e| serde::de::Error::custom(format!("Failed to get username: {e}")))?;

        Ok(StockModel::new(
            id,
            username,
            code,
            shares,
            buy_price,
            date,
            current_price,
        ))
    }
}