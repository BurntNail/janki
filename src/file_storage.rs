use crate::{game::AnkiDB, storage::Storage};
use serde_json::{from_str, to_string};
use thiserror::Error;

///A struct implementing [`Storage`] for [`std::fs::File`] that uses [`std::fs::read_to_string`] and [`std::fs::write`]
#[derive(Debug, Clone)]
pub struct NamedFileStorage(pub String);

///Struct using `thiserror` that contains all errors for [`NamedFileStorage`]
#[derive(Error, Debug)]
pub enum NamedFileStorageError {
    ///Error reading a file - comes from [`std::io::Error`]
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),

    ///Error with `serde_json` - comes from [`serde_json::Error`]
    #[error("serde_json error: {0}")]
    SJError(#[from] serde_json::Error),
}

impl<S: Into<String>> From<S> for NamedFileStorage {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Storage for NamedFileStorage {
    type ErrorType = NamedFileStorageError;

    #[instrument]
    fn read_db(&self) -> Result<AnkiDB, Self::ErrorType> {
        trace!("Reading from FileStorage");
        let contents = std::fs::read_to_string(&self.0).unwrap_or_else(|_e| "[]".into());
        Ok(from_str(&contents)?)
    }

    #[instrument]
    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType> {
        trace!("Writing to FileStorage");
        Ok(std::fs::write(&self.0, to_string(db)?)?)
    }
}
