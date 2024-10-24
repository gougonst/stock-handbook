use bson::{from_bson, Bson};
use chrono::prelude::*;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use log::debug;

use crate::constants;

#[derive(Serialize, Debug)]
pub struct Stock {
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
}

impl Stock {
    pub fn new(
        id: Option<String>, 
        username: String, 
        code: String, 
        shares: i32, 
        buy_price: f64, 
        date: DateTime<Utc>, 
        current_price: f64
    ) -> Stock {
        Stock {
            id, 
            username, 
            code, 
            shares, 
            buy_price, 
            date, 
            current_price, 
            fee: Stock::calc_fee(shares, buy_price), 
        }
    }

    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    fn calc_fee(shares: i32, buy_price: f64) -> i32 {
        let fee = (shares as f64 * buy_price * 0.001425).trunc() as i32;
        match fee <= 20 {
            true => {
                20
            }, 
            false => {
                fee
            }
        }
    }
}

impl<'de> Deserialize<'de> for Stock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let doc = bson::Document::deserialize(deserializer)?;

        debug!("Doc: {:?}", doc);

        let id = doc.get_object_id(constants::STOCK_COLL_ID_COL)
            .map(|oid| oid.to_hex())
            .ok();
        let buy_price = doc.get_f64(constants::STOCK_COLL_BUY_PRICE_COL)
            .map_err(|e| serde::de::Error::custom(
                format!("Failed to get buy price: {e}")
            ))?;
        let code = doc.get_str(constants::STOCK_COLL_CODE_COL)
            .map(|s| s.to_string())
            .map_err(|e| serde::de::Error::custom(
                format!("Failed to get code: {e}")
            ))?;
        let current_price = doc.get_f64(constants::STOCK_COLL_CURRENT_PRICE_COL)
            .map_err(|e| serde::de::Error::custom(
                format!("Failed to get current price: {e}")
            ))?;
        let date = doc.get_datetime(constants::STOCK_COLL_DATE_COL)
            .map(|dt| dt.to_chrono())
            .map_err(|e| serde::de::Error::custom(
                format!("Failed to get date: {e}")
            ))?;
        let shares = doc.get_i32(constants::STOCK_COLL_SHARES_COL)
            .map_err(|e| serde::de::Error::custom(
                format!("Failed to get shares: {e}")
            ))?;
        let username = doc.get_str(constants::STOCK_COLL_USERNAME_COL)
            .map(|s| s.to_string())
            .map_err(|e| serde::de::Error::custom(
                format!("Failed to get username: {e}")
            ))?;

        Ok(Stock::new(
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
