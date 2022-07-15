use crate::{game::AnkiDB, storage::Storage as JStorage};
use eframe::Storage as EStorage;
use serde_json::{from_str, to_string};

impl JStorage for &dyn EStorage {
    type ErrorType = serde_json::Error;

    fn read_db(&self) -> Result<AnkiDB, Self::ErrorType> {
        Ok(from_str(
            &self.get_string("db").unwrap_or_else(|| "[]".into()),
        )?)
    }

    fn write_db(&mut self, _db: &AnkiDB) -> Result<(), Self::ErrorType> {
        Ok(())
    }
}

impl JStorage for &mut dyn EStorage {
    type ErrorType = serde_json::Error;

    fn read_db(&self) -> Result<AnkiDB, Self::ErrorType> {
        Ok(from_str(
            &self.get_string("db").unwrap_or_else(|| "[]".into()),
        )?)
    }

    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType> {
        Ok(self.set_string("db", to_string(db)?))
    }
}
