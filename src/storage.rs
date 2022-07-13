use crate::{db::AnkiDB, errors::JankiError};
use serde_json::{from_str, to_string};
use std::error::Error;

pub trait Storage {
    type ErrorType: Error;

    fn read_db(&mut self) -> Result<AnkiDB, Self::ErrorType>;
    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType>;
    fn exit_application(&mut self) {}
}

#[derive(Debug, Clone)]
pub struct FileStorage(pub String);

impl<S: Into<String>> From<S> for FileStorage {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Storage for FileStorage {
    type ErrorType = JankiError;

    fn read_db(&mut self) -> Result<AnkiDB, Self::ErrorType> {
        let contents = std::fs::read_to_string(&self.0).unwrap_or_else(|_e| "[]".into());
        Ok(from_str(&contents)?)
    }

    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType> {
        Ok(std::fs::write(&self.0, to_string(db)?)?)
    }
}
