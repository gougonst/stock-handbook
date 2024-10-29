use crate::database::repository_error::RepositoryError;
use crate::{constants, models::stock_record_model::StockRecordModel};
use futures::TryStreamExt;
use log::{debug, error};
use mongodb::{
    bson::{self, doc, Document},
    Collection, Database,
};
use std::sync::Arc;

pub struct StockRecordRepository {
    db: Arc<Database>,
}

impl StockRecordRepository {
    pub fn new(db: Arc<Database>) -> Self {
        StockRecordRepository { db }
    }

    pub async fn get_stock_records(&self, username: &str) -> Result<Vec<StockRecordModel>, RepositoryError> {
        let record_coll: Collection<Document> = self.db.collection(constants::RECORD_COLL_NAME);

        let mut result = record_coll
            .find(doc! {constants::RECORD_COLL_USERNAME_COL: username})
            .await
            .map_err(RepositoryError::DatabaseError)?;

        let mut records: Vec<StockRecordModel> = Vec::new();
        while let Some(record_doc) = result
            .try_next()
            .await
            .map_err(RepositoryError::DatabaseError)?
        {
            let record: StockRecordModel = bson::from_document(record_doc).map_err(|e| {
                error!("BsonDeserializaError: {:?}", e);
                RepositoryError::BsonDeserializeError(e)
            })?;
            records.push(record);
        }
        
        Ok(records)
    }

    pub async fn add_stock_record(&self, record: &StockRecordModel) -> Result<String, RepositoryError> {
        let record_coll: Collection<Document> = self.db.collection(constants::RECORD_COLL_NAME);

        let mut record_doc =
            bson::to_document(&record).map_err(RepositoryError::BsonSerializeError)?;
        // Fee should be calculated by backend
        record_doc.remove("fee");
        record_doc.remove("principal");
        let res = record_coll.insert_one(record_doc).await?;
        let new_id = res
            .inserted_id
            .as_object_id()
            .map(|oid| oid.to_hex())
            .unwrap();
        Ok(new_id)
    }
}
