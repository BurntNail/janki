#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::missing_docs_in_private_items
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]
//! Welcome to Janki!
//! Just a really janky version of Anki - the popular spaced repetition learning tool.

///A module for the [`either::Either`] enum
pub mod either;
///A module to deal with errors and hold [`errors::JankiError`]
pub mod errors;
///A module to hold [`game::AnkiGame`]
pub mod game;
///A module to hold [`item::Item`], [`item::ItemGuard`] and [`item::Fact`]
pub mod item;
///A module to hold the [`storage::Storage`] trait
pub mod storage;

#[cfg(feature = "file_storage" )]
///A module to hold the [`file_storage::FileStorage`] struct
mod file_storage;

pub use serde_json::error::Error as SerdeJsonError;
