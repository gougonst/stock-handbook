use crate::models::user::{User, UserModel};
use async_trait::async_trait;
use mongodb::bson::de::Error as BsonDeError;
use mongodb::bson::ser::Error as BsonSeError;
use mongodb::error::Error as MongoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("Database error")]
    DatabaseError(#[from] MongoError),
    #[error("Bson deserialize error")]
    BsonDeserializeError(#[from] BsonDeError),
    #[error("Bson serialize error")]
    BsonSerializeError(#[from] BsonSeError),
    #[error("User not found")]
    UserNotFound,
}

#[async_trait]
pub trait UserRepository {
    async fn check_user(&self, user: &UserModel) -> Result<bool, UserRepositoryError>;
    async fn create_user(&self, user: &UserModel) -> Result<bool, UserRepositoryError>;
}
