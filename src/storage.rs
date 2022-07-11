use crate::{db::AnkiDB, errors::JankiError};
use serde_json::{from_str, to_string};
use std::error::Error;
use std::io::{Read, Write};

pub trait Storage {
    type ErrorType: Error;

    fn read_db(&mut self) -> Result<AnkiDB, Self::ErrorType>;
    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType>;
    fn exit_application(&mut self) {}
}

impl<T: Read + Write> Storage for T {
    type ErrorType = JankiError;

    fn read_db(&mut self) -> Result<AnkiDB, Self::ErrorType> {
        let mut contents = String::new();
        self.read_to_string(&mut contents)?;
        Ok(from_str(&contents)?)
    }

    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType> {
        Ok(write!(self, "{}", to_string(db)?).map(|_| ())?)
    }
}
