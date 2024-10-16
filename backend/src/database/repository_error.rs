use mongodb::bson::de::Error as BsonDeError;
use mongodb::bson::ser::Error as BsonSeError;
use mongodb::error::Error as MongoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Database error")]
    DatabaseError(#[from] MongoError),
    #[error("Bson deserialize error")]
    BsonDeserializeError(#[from] BsonDeError),
    #[error("Bson serialize error")]
    BsonSerializeError(#[from] BsonSeError),
    #[error("User not found")]
    UserNotFound,
}
