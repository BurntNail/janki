# Janki
 
 ## CI/CD
 Janki currently has 2 CI files, one for building example executables for windows, and one for checking code:
 
 
 - Checking Code: [![Quickstart](https://github.com/Epacnoss/janki/actions/workflows/ci.yml/badge.svg)](https://github.com/Epacnoss/janki/actions/workflows/ci.yml)
 
 - Build: [![Build Windows](https://github.com/Epacnoss/janki/actions/workflows/build.yml/badge.svg)](https://github.com/Epacnoss/janki/actions/workflows/build.yml)

## Binary Use
CLI or GUI using egui - if I need to make traits feel free to add a Issue/PR.
The CLI is pretty simplistic, and the GUI makes use of more library features (except for `ItemGuard` godammit).

These aren't documented, but shouldn't be *too* tricky to understand. If need be, again open an Issue/PR.

 - CLI: `cargo run --example janki_cli`
 - GUI w/ egui: `cargo run --example janki_egui`
 - GUI w/ dioxus: `cargo run --example janki_dioxus` (currently not supported as still in development)

Plans for more versions later:
 - TUI
 - EVEN MORE UI LIBRARIES

## Library Use
This should be on crates.io, and is decently documented, at least for one of my projects.
Feel free to use the binaries as examples.

## TODO/Future Plans:
 - generics for the `Fact` struct
 - Docu-Tests/Examples
 - The new binaries
 - Better Cargo.toml Keywords and Categories
