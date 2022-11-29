//! # Example
//!
//! ```rust
//! use libloading_helper::{library, LibrarySymbol};
//!
//! #[library]
//! mod ffi {
//!     extern "C" {
//!         pub fn ffi_all(a: i32, b: u64, ...) -> i32;
//!
//!         pub static A: i32;
//!     }
//! }
//!
//! assert_eq!(ffi::ffi_all::NAME, "ffi_all");
//! assert_eq!(ffi::A::NAME, "A");
//!
//! assert_eq!(ffi::ffi_all::SYMBOL, b"ffi_all\0");
//! assert_eq!(ffi::A::SYMBOL, b"A\0");
//!
//! assert_eq!(std::any::type_name::<<ffi::ffi_all as LibrarySymbol>::Type>(), "unsafe extern \"C\" fn(i32, u64, ...) -> i32");
//! assert_eq!(std::any::type_name::<<ffi::A as LibrarySymbol>::Type>(), "*mut i32");
//! ```

pub use libloading_helper_macro::library;

pub trait LibrarySymbol {
    const NAME: &'static str;
    const SYMBOL: &'static [u8];
    type Type;
}
