use crate::database::user_repository::UserRepository;
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<UserRepository>,
}
