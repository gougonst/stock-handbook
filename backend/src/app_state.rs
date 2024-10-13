use crate::database::mongo_repository;
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<mongo_repository::MongoUserRepository>,
}
