use crate::database::stock_record_repository::StockRecordRepository;
use crate::database::user_repository::UserRepository;
use std::sync::Arc;

pub struct AppState {
    pub user_repo: Arc<UserRepository>,
    pub record_repo: Arc<StockRecordRepository>,
}
