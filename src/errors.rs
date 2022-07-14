use crate::SerdeJsonError;
use thiserror::Error;

///Struct using `thiserror` that contains all errors for Janki
#[derive(Error, Debug)]
pub enum JankiError {
    ///Error reading a file - comes from [`std::io::Error`]
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
    ///Error with serde_json - comes from [`SerdeJsonError`]
    #[error("serde_json error: {0}")]
    SJError(#[from] SerdeJsonError),
}
