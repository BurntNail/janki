use crate::{errors::JankiError, game::AnkiDB, storage::Storage};
use serde_json::{from_str, to_string};

///A struct implementing [`Storage`] for [`std::fs::File`] that uses [`std::fs::read_to_string`] and [`std::fs::write`]
#[derive(Debug, Clone)]
pub struct FileStorage(pub String);

impl<S: Into<String>> From<S> for FileStorage {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Storage for FileStorage {
    type ErrorType = JankiError;

    fn read_db(&self) -> Result<AnkiDB, Self::ErrorType> {
        let contents = std::fs::read_to_string(&self.0).unwrap_or_else(|_e| "[]".into());
        Ok(from_str(&contents)?)
    }

    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType> {
        Ok(std::fs::write(&self.0, to_string(db)?)?)
    }
}
