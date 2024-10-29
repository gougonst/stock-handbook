use crate::database::repository_error::RepositoryError;
use crate::{constants, models::stock::Stock, models::inventory::Inventory};
use bson::oid::ObjectId;
use futures::TryStreamExt;
use log::{debug, error};
use mongodb::{
    bson::{self, doc, Document},
    Collection, Database,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct StockRepository {
    db: Arc<Database>,
}

impl StockRepository {
    pub fn new(db: Arc<Database>) -> Self {
        StockRepository { db }
    }

    pub async fn get_stocks(&self, username: &str) -> Result<HashMap<String, Inventory>, RepositoryError> {
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
            let stock: Stock = bson::from_document(stock_doc).map_err(|e| {
                error!("BsonDeserializaError: {:?}", e);
                RepositoryError::BsonDeserializeError(e)
            })?;
            stocks.push(stock);
        }

        Ok(self.create_inventories(stocks))
    }

    fn create_inventories(&self, stocks: Vec<Stock>) -> HashMap<String, Inventory> {
        let mut grouped_stocks = HashMap::new();

        for stock in stocks {
            let code = stock.get_code();

            if !grouped_stocks.contains_key(code) {
                grouped_stocks.insert(code.to_string(), Inventory::from_stock(&stock));
            } else {
                if let Some(grouped_stock) = grouped_stocks.get_mut(code) {
                    grouped_stock.add_stock(&stock);
                }
            }
        }
        grouped_stocks
    }

    pub async fn add_stock(&self, stock: &Stock) -> Result<String, RepositoryError> {
        let stock_coll: Collection<Document> = self.db.collection(constants::STOCK_COLL_NAME);

        let mut stock_doc =
            bson::to_document(&stock).map_err(RepositoryError::BsonSerializeError)?;
        // Fee should be calculated by backend
        stock_doc.remove("fee");
        stock_doc.remove("principal");
        let res = stock_coll.insert_one(stock_doc).await?;
        let new_id = res
            .inserted_id
            .as_object_id()
            .map(|oid| oid.to_hex())
            .unwrap();
        Ok(new_id)
    }

    pub async fn delete_stocks(&self, id: &str) -> Result<bool, RepositoryError> {
        let stock_coll: Collection<Document> = self.db.collection(constants::STOCK_COLL_NAME);

        let oid = ObjectId::parse_str(id).map_err(RepositoryError::ObjectIdError)?;
        let res = stock_coll
            .delete_one(doc! {
                "_id": oid
            })
            .await?;

        Ok(!(res.deleted_count == 0))
    }
}
