use thiserror::Error;

#[derive(Error, Debug)]
pub enum JankiError {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    SJError(#[from] serde_json::Error),
}
