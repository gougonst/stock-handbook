use chrono::prelude::*;
use mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Deserialize, Serialize, Debug)]
pub struct Stock {
    username: String,
    code: String,
    shares: i32,
    buy_price: f32,
    principal: i32,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    date: DateTime<Utc>,
    current_price: f32,
}

impl Stock {
    pub fn new(
        username: String,
        code: String,
        shares: i32,
        buy_price: f32,
        principal: i32,
        date: DateTime<Utc>,
        current_price: f32,
    ) -> Stock {
        Stock {
            username,
            code,
            shares,
            buy_price,
            principal,
            date,
            current_price,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn shares(&self) -> &i32 {
        &self.shares
    }

    pub fn buy_price(&self) -> &f32 {
        &self.buy_price
    }

    pub fn principal(&self) -> &i32 {
        &self.principal
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn current_price(&self) -> &f32 {
        &self.current_price
    }
}
