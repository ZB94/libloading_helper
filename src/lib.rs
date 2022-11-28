//! # Example
//!
//! ```rust
//! use libloading_helper::{library, LibrarySymbol};
//!
//! #[library]
//! mod ffi {
//!     extern "C" {
//!         pub fn ffi();
//!
//!         pub fn ffi_args(a: i32, b: u64, ...);
//!
//!         pub fn ffi_ret() -> i32;
//!
//!         pub fn ffi_all(a: i32, b: u64, ...) -> i32;
//!     }
//! }
//!
//! assert_eq!(ffi::ffi::NAME, "ffi");
//! assert_eq!(ffi::ffi_args::NAME, "ffi_args");
//! assert_eq!(ffi::ffi_ret::NAME, "ffi_ret");
//! assert_eq!(ffi::ffi_all::NAME, "ffi_all");
//!
//! assert_eq!(ffi::ffi::SYMBOL, b"ffi\0");
//! assert_eq!(ffi::ffi_args::SYMBOL, b"ffi_args\0");
//! assert_eq!(ffi::ffi_ret::SYMBOL, b"ffi_ret\0");
//! assert_eq!(ffi::ffi_all::SYMBOL, b"ffi_all\0");
//!
//! assert_eq!(std::any::type_name::<<ffi::ffi as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn()");
//! assert_eq!(std::any::type_name::<<ffi::ffi_args as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn(i32, u64, ...)");
//! assert_eq!(std::any::type_name::<<ffi::ffi_ret as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn() -> i32");
//! assert_eq!(std::any::type_name::<<ffi::ffi_all as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn(i32, u64, ...) -> i32");
//! ```

pub use libloading_helper_macro::library;

pub trait LibrarySymbol {
    const NAME: &'static str;
    const SYMBOL: &'static [u8];
    type Type;
}
