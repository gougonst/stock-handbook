use crate::{constants, models::user::User};
use std::sync::Arc;
// use crate::models::stock::{Stock}
use mongodb::bson::de::Error as BsonDeError;
use mongodb::bson::ser::Error as BsonSeError;
use mongodb::error::Error as MongoError;
use mongodb::{
    bson::{self, doc, to_document, Document},
    Collection, Database,
};
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

pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    // Pass db parameter so that it can connect to MockDatabase
    pub fn new(db: Arc<Database>) -> Self {
        UserRepository { db }
    }

    pub async fn check_user(&self, user: &User) -> Result<bool, UserRepositoryError> {
        let db_user: User = match self.get_user(user.username()).await {
            Ok(Some(db_user)) => db_user,
            Ok(None) => return Err(UserRepositoryError::UserNotFound),
            Err(e) => return Err(e),
        };

        Ok(user.password() == db_user.password())
    }

    pub async fn create_user(&self, user: &User) -> Result<bool, UserRepositoryError> {
        match self.get_user(user.username()).await? {
            Some(_) => Ok(false),
            None => {
                let user_coll: Collection<Document> = self.db.collection(constants::USER_COLL_NAME);

                let user_doc =
                    bson::to_document(&user).map_err(UserRepositoryError::BsonSerializeError)?;
                user_coll.insert_one(user_doc).await?;
                Ok(true)
            }
        }
    }

    async fn get_user(&self, username: &str) -> Result<Option<User>, UserRepositoryError> {
        let user_coll: Collection<Document> = self.db.collection(constants::USER_COLL_NAME);

        let user_doc = user_coll
            .find_one(doc! {constants::USER_COLL_USERNAME_COL: username})
            .await
            .map_err(UserRepositoryError::DatabaseError)?;

        match user_doc {
            Some(user_doc) => {
                let user: User = bson::from_document(user_doc)
                    .map_err(UserRepositoryError::BsonDeserializeError)?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
