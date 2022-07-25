use crate::{game::AnkiDB, storage::Storage as JStorage};
use eframe::Storage as EStorage;
use serde_json::{from_str, to_string};

impl JStorage for &dyn EStorage {
    type ErrorType = serde_json::Error;

    #[instrument(skip(self))]
    fn read_db(&self) -> Result<AnkiDB, Self::ErrorType> {
        trace!("Reading &dyn EStorage");
        Ok(from_str(
            &self.get_string("db").unwrap_or_else(|| "[]".into()),
        )?)
    }

    #[instrument(skip(self))]
    fn write_db(&mut self, _db: &AnkiDB) -> Result<(), Self::ErrorType> {
        error!("Cannot write to an &dyn EStorage due to mutability.");
        Ok(())
    }
}

impl JStorage for &mut dyn EStorage {
    type ErrorType = serde_json::Error;

    #[instrument(skip(self))]
    fn read_db(&self) -> Result<AnkiDB, Self::ErrorType> {
        trace!("Reading &mut dyn EStorage");
        Ok(from_str(
            &self.get_string("db").unwrap_or_else(|| "[]".into()),
        )?)
    }

    #[instrument(skip(self, db))]
    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType> {
        trace!("Writing to &mut dyn EStorage");
        Ok(self.set_string("db", to_string(db)?))
    }
}
