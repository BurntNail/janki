# Janki

## Binary Use
CLI or GUI using egui - if I need to make traits feel free to add a Issue/PR.
The CLI is pretty simplistic, and the GUI makes use of more library features (except for `ItemGuard` godammit).

These aren't documented, but shouldn't be *too* tricky to understand. If need be, again open an Issue/PR.

 - CLI: `cargo run --example janki_cli`
 - GUI w/ egui: `cargo run --example janki_egui`
 - GUI w/ druid: `cargo run --example janki_druid`

Plans for more versions later:
 - TUI
 - egui WASM support

## Library Use
This should be on crates.io, and is decently documented, at least for one of my projects.
Feel free to use the binaries as examples.

## TODO/Future Plans:
 - generics for the `Fact` struct
 - Docu-Tests/Examples
 - The new binaries
 - Better Cargo.toml Keywords and Categories