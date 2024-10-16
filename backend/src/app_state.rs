use crate::database::stock_repository::StockRepository;
use crate::database::user_repository::UserRepository;
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<UserRepository>,
    pub stock_repo: Arc<StockRepository>,
}
