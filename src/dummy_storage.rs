use crate::{game::AnkiDB, storage::Storage};

///A dummy database - works only in memory
#[derive(Default)]
pub struct DummyStorage(AnkiDB);

impl Storage for DummyStorage {
    type ErrorType = ();

    fn read_db(&self) -> Result<AnkiDB, Self::ErrorType> {
        Ok(self.0.clone())
    }

    fn write_db(&mut self, db: &AnkiDB) -> Result<(), Self::ErrorType> {
        self.0 = db.clone();
        Ok(())
    }
}

///Trait for [`Storage`] that implements methods that take `dyn` trait objects over owned `self`s.
pub trait DynStorage<E: std::fmt::Debug> {
    ///Reads the database from `S` and sets the owned database from `&mut self` to that read in database.
    fn read_custom(&mut self, s: &dyn Storage<ErrorType = E>) -> Result<(), E>;

    ///Writes the owned database to `S`
    fn write_custom(&mut self, s: &mut dyn Storage<ErrorType = E>) -> Result<(), E>;

    ///Exits the application
    fn exit_custom(&mut self, s: &mut dyn Storage<ErrorType = E>);
}
