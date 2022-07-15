#[cfg(feature = "serde_json")]
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

///Struct using `thiserror` that contains all errors for Janki
#[derive(Error, Debug)]
pub enum JankiError {
    ///Error reading a file - comes from [`std::io::Error`]
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),

    #[cfg(feature = "serde_json")]
    ///Error with serde_json - comes from [`SerdeJsonError`]
    #[error("serde_json error: {0}")]
    SJError(#[from] SerdeJsonError),
}
