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
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions
)]
//! Welcome to Janki!
//! Just a really janky version of Anki - the popular spaced repetition learning tool.

#[macro_use]
extern crate tracing;

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
///A module to hold the [`file_storage::NamedFileStorage`] struct
pub mod file_storage;

///Module to deal with importing and exporting to and from CSV files
pub mod csv;

///Re-export the [`serde`] crate
pub mod serde {
    pub use serde::*;
}
///Re-export the [`serde_json`] crate
pub mod serde_json {
    pub use serde_json::*;
}

///Quick wrapper for [`std::io::Write`] for use with [`String`]
#[cfg(test)]
mod string_wrapper {
    use std::{
        io::{ErrorKind, Write},
        ops::Deref,
    };

    #[derive(Default, Debug)]
    pub struct StringWrapper(String);

    impl StringWrapper {
        pub fn to_inner(self) -> String {
            self.0
        }
    }

    impl Deref for StringWrapper {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Write for StringWrapper {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let new_string = match String::from_utf8(buf.into()) {
                Ok(s) => s,
                Err(e) => return Err(std::io::Error::new(ErrorKind::InvalidData, e)),
            };

            self.0 += &new_string;

            Ok(new_string.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
