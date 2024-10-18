use crate::database::repository_error::RepositoryError;
use crate::{constants, models::stock::Stock};
use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, Document},
    Collection, Database,
};
use log::{debug, error};
use std::sync::Arc;

pub struct StockRepository {
    db: Arc<Database>,
}

impl StockRepository {
    pub fn new(db: Arc<Database>) -> Self {
        StockRepository { db }
    }

    pub async fn get_stocks(&self, username: &str) -> Result<Vec<Stock>, RepositoryError> {
        let stock_coll: Collection<Document> = self.db.collection(constants::STOCK_COLL_NAME);

        let mut result = stock_coll
            .find(doc! {constants::STOCK_COLL_USERNAME_COL: username})
            .await
            .map_err(RepositoryError::DatabaseError)?;

        let mut stocks: Vec<Stock> = Vec::new();
        while let Some(stock_doc) = result
            .try_next()
            .await
            .map_err(RepositoryError::DatabaseError)?
        {
            debug!("Hello");
            let stock: Stock =
                bson::from_document(stock_doc).map_err(|e| {
                    error!("BsonDeserializaError: {:?}", e);
                    RepositoryError::BsonDeserializeError(e)
                })?;
            stocks.push(stock);
        }

        Ok(stocks)
    }

    pub async fn add_stocks(&self, stock: &Stock) -> Result<bool, RepositoryError> {
        let stock_coll: Collection<Document> = self.db.collection(constants::STOCK_COLL_NAME);

        let stock_doc = 
            bson::to_document(&stock).map_err(RepositoryError::BsonSerializeError)?;
        stock_coll.insert_one(stock_doc).await?;
        Ok(true)
    }

    pub async fn delete_stocks(&self, stock: &str) -> Result<bool, RepositoryError> {
        Ok(false)
    }
}
