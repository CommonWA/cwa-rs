//! Rust API library for the [CommonWA specification](https://github.com/CommonWA/cwa-spec).

#![feature(wasm_import_module)]

#[macro_use]
mod utils;

pub mod raw;
pub mod log;
pub mod env;
pub mod runtime;
