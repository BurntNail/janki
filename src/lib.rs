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

///A module to hold a the [`dummy_storage::DummyStorage`] struct
pub mod dummy_storage;
///A module for the [`either::Either`] enum
pub mod either;
///A module to hold [`game::AnkiGame`]
pub mod game;
///A module to hold [`item::Item`], [`item::ItemGuard`] and [`item::Fact`]
pub mod item;
///A module to hold the [`storage::Storage`] trait
pub mod storage;

#[cfg(feature = "eframe_storage")]
///A module to implement [`storage::Storage`] for [`eframe::Storage`]
pub mod eframe_storage;
#[cfg(feature = "file_storage")]
///A module to hold the [`file_storage::NamedFileStorage`] struct
pub mod file_storage;

#[cfg(feature = "serde_derive_structs")]
///Re-export the [`serde`] crate
pub mod serde {
    pub use serde::*;
}
#[cfg(feature = "serde_json_stuff")]
///Re-export the [`serde_json`] crate
pub mod serde_json {
    pub use serde_json::*;
}
