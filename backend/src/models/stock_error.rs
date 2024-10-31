use thiserror::Error;

#[derive(Error, Debug)]
pub enum StockError {
    #[error("Transaction error")]
    TransactionError,
}
