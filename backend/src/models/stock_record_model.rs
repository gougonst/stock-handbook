use chrono::prelude::*;
use chrono::Utc;
use core::fmt;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::constants;
use crate::constants::ACTION_ADD;
use crate::constants::ACTION_DELETE;

#[derive(Serialize, Deserialize, Debug)]
pub enum StockRecordAction {
    Add,
    Delete,
}

impl FromStr for StockRecordAction {
    type Err = fmt::Error;

    fn from_str(action: &str) -> Result<Self, <Self as FromStr>::Err> {
        match action {
            ACTION_ADD => Ok(StockRecordAction::Add),
            ACTION_DELETE => Ok(StockRecordAction::Delete),
            _ => Err(fmt::Error),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct StockRecordModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    action: StockRecordAction,
    username: String,
    code: String,
    shares: i32,
    transaction_price: f64,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    date: DateTime<Utc>,
    current_price: f64,
    fee: i32,
    principal: f64,
}

impl StockRecordModel {
    pub fn new(
        id: Option<String>,
        action: StockRecordAction,
        username: String,
        code: String,
        shares: i32,
        transaction_price: f64,
        date: DateTime<Utc>,
        current_price: f64,
    ) -> StockRecordModel {
        StockRecordModel {
            id,
            action,
            username,
            code,
            shares,
            transaction_price,
            date,
            current_price,
            fee: StockRecordModel::calc_fee(shares, transaction_price),
            principal: StockRecordModel::calc_principal(shares, transaction_price),
        }
    }

    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    pub fn get_action(&self) -> &StockRecordAction {
        &self.action
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

    pub fn get_transaction_price(&self) -> f64 {
        self.transaction_price
    }

    pub fn calc_principal(shares: i32, transaction_price: f64) -> f64 {
        shares as f64 * transaction_price
    }

    fn calc_fee(shares: i32, transaction_price: f64) -> i32 {
        let fee = (StockRecordModel::calc_principal(shares, transaction_price) * 0.001425).trunc() as i32;
        match fee <= 20 {
            true => 20,
            false => fee,
        }
    }

    fn calc_transaction_tax(shares: i32, transaction_price: f64) -> i32 {
        let tax = (StockRecordModel::calc_principal(shares, transaction_price) * 0.003).trunc() as i32;
        tax
    }
}

impl<'de> Deserialize<'de> for StockRecordModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let doc = bson::Document::deserialize(deserializer)?;

        let id = doc
            .get_object_id(constants::RECORD_COLL_ID_COL)
            .map(|oid| oid.to_hex())
            .ok();
        let action: StockRecordAction = StockRecordAction::from_str(
            doc.get_str(constants::RECORD_COLL_ACTION_COL)
                .map_err(|e| serde::de::Error::custom(format!("Failed to get action: {e}")))?,
        )
        .map_err(|e| serde::de::Error::custom(format!("Failed to parse action: {e}")))?;

        let transaction_price = doc
            .get_f64(constants::RECORD_COLL_TRANSACTION_PRICE_COL)
            .map_err(|e| serde::de::Error::custom(format!("Failed to get buy price: {e}")))?;
        let code = doc
            .get_str(constants::RECORD_COLL_CODE_COL)
            .map(|s| s.to_string())
            .map_err(|e| serde::de::Error::custom(format!("Failed to get code: {e}")))?;
        let current_price = doc
            .get_f64(constants::RECORD_COLL_CURRENT_PRICE_COL)
            .map_err(|e| serde::de::Error::custom(format!("Failed to get current price: {e}")))?;
        let date = doc
            .get_datetime(constants::RECORD_COLL_DATE_COL)
            .map(|dt| dt.to_chrono())
            .map_err(|e| serde::de::Error::custom(format!("Failed to get date: {e}")))?;
        let shares = doc
            .get_i32(constants::RECORD_COLL_SHARES_COL)
            .map_err(|e| serde::de::Error::custom(format!("Failed to get shares: {e}")))?;
        let username = doc
            .get_str(constants::RECORD_COLL_USERNAME_COL)
            .map(|s| s.to_string())
            .map_err(|e| serde::de::Error::custom(format!("Failed to get username: {e}")))?;

        Ok(StockRecordModel::new(
            id,
            action,
            username,
            code,
            shares,
            transaction_price,
            date,
            current_price,
        ))
    }
}
