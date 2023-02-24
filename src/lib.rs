#![doc = include_str!("../README.md")]

pub use libloading;
#[cfg(unix)]
pub use libloading::os::unix::Symbol as RawSymbol;
#[cfg(windows)]
pub use libloading::os::windows::Symbol as RawSymbol;
pub use libloading::{library_filename, Error, Library, Symbol};
pub use libloading_helper_macro::library;
