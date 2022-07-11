#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

pub mod db;
pub mod errors;
pub mod item;
pub mod storage;
pub use serde_json::error::Error as SerdeJsonError;
