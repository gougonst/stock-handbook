use crate::database::repository::{UserRepository, UserRepositoryError};
use crate::models::user::User;
use crate::{constants, models::user::UserModel};
use async_trait::async_trait;
use mongodb::{
    bson::{self, doc, to_document, Document},
    Collection, Database,
};
use std::sync::Arc;

pub struct MongoUserRepository {
    db: Arc<Database>,
}

impl MongoUserRepository {
    // Pass db parameter so that it can connect to MockDatabase
    pub fn new(db: Arc<Database>) -> Self {
        MongoUserRepository { db }
    }

    async fn get_user(&self, username: &str) -> Result<Option<Box<dyn User>>, UserRepositoryError> {
        let user_coll: Collection<Document> = self.db.collection(constants::USER_COLL_NAME);

        let user_doc = user_coll
            .find_one(doc! {constants::USER_COLL_USERNAME_COL: username})
            .await
            .map_err(UserRepositoryError::DatabaseError)?;

        match user_doc {
            Some(user_doc) => {
                let user: UserModel = bson::from_document(user_doc)
                    .map_err(UserRepositoryError::BsonDeserializeError)?;
                Ok(Some(Box::new(user)))
            }
            None => Ok(None),
        }
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn check_user(&self, user: &UserModel) -> Result<bool, UserRepositoryError> {
        let db_user: Box<dyn User> = match self.get_user(user.username()).await {
            Ok(Some(db_user)) => db_user,
            Ok(None) => return Err(UserRepositoryError::UserNotFound),
            Err(e) => return Err(e),
        };

        Ok(user.password() == db_user.password())
    }

    async fn create_user(&self, user: &UserModel) -> Result<bool, UserRepositoryError> {
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
}
