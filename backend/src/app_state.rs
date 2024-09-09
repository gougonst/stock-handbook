use std::sync::Arc;
use crate::database::mongo_repository;

pub struct AppState {
    pub user_repo: Arc<mongo_repository::MongoUserRepository>, 
}
